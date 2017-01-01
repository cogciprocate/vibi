#![allow(unused_imports)]

extern crate vibi;

use vibi::window;
use vibi::bismit::{Cortex, Subcortex, Flywheel, CorticalAreaSettings};
use vibi::bismit::map::{self, LayerTags, LayerMapKind, LayerMapScheme, LayerMapSchemeList,
    AreaScheme, AreaSchemeList, CellScheme, FilterScheme, InputScheme, AxonKind, LayerKind};

// Test stuff:
use vibi::bismit::encode::HexMoldTest;
use vibi::bismit::{TestScNucleus};

fn main() {
    use std::thread;
    use std::sync::mpsc;

    let (command_tx, command_rx) = mpsc::channel();
    let (request_tx, request_rx) = mpsc::channel();
    let (response_tx, response_rx) = mpsc::channel();

    let th_flywheel = thread::Builder::new().name("flywheel".to_string()).spawn(move || {
        // let mut flywheel = Flywheel::from_blueprint(command_rx, define_lm_schemes(),
        //     define_a_schemes(), Some(ca_settings()));
        let mut cortex = Cortex::new(define_lm_schemes(), define_a_schemes(), Some(ca_settings()))
            .sub(Subcortex::new().nucleus(Box::new(TestScNucleus::new("m0"))));

        let ia_idx = cortex.thal().ext_pathway_idx(&"v0".to_owned()).unwrap();
        cortex.thal_mut().ext_pathway(ia_idx).unwrap().specify_encoder(Box::new(
            HexMoldTest::new(6 * DST_AREA_SCL as i8, (AREA_SIDE, AREA_SIDE))
        )).unwrap();

        let mut flywheel = Flywheel::new(cortex, command_rx);
        flywheel.add_req_res_pair(request_rx, response_tx);
        flywheel.spin();
    }).expect("Error creating 'flywheel' thread");

    let th_win = thread::Builder::new().name("win".to_string()).spawn(move || {
        window::Window::open(command_tx, request_tx, response_rx);
    }).expect("Error creating 'win' thread");

    if let Err(e) = th_win.join() { println!("th_win.join(): Error: '{:?}'", e); }
    if let Err(e) = th_flywheel.join() { println!("th_flywheel.join(): Error: '{:?}'", e); }
}


fn define_lm_schemes() -> LayerMapSchemeList {
    const MOTOR_UID: u32 = 101;

    LayerMapSchemeList::new()
        .lmap(LayerMapScheme::new("v1_lm", LayerMapKind::Cortical)
            //.layer("test_noise", 1, map::DEFAULT, LayerKind::Axonal(Spatial))
            .axn_layer("motor_ctx", map::NS_IN | LayerTags::uid(MOTOR_UID), AxonKind::Horizontal)
            // .axn_layer("olfac", map::NS_IN | LayerTags::with_uid(OLFAC_UID), Horizontal)
            // .axn_layer("eff_in", map::FB_IN, AxonKind::Spatial)
            .axn_layer("aff_in", map::FF_IN, AxonKind::Spatial)
            .axn_layer("unused", map::UNUSED_TESTING, AxonKind::Spatial)
            .layer("mcols", 1, map::FF_FB_OUT, CellScheme::minicolumn("iv", "iii"))
            .layer("iv_inhib", 0, map::DEFAULT, CellScheme::inhibitory(4, "iv"))

            .layer("iv", 1, map::PSAL,
                CellScheme::spiny_stellate(6, vec!["aff_in"], 000, 10))


            .layer("iii", 1, map::PTAL,
                CellScheme::pyramidal(1, 3, vec!["iii"], 500, 10)
                    // .apical(vec!["eff_in"/*, "olfac"*/], 18)
                )
        )
        .lmap(LayerMapScheme::new("v0_lm", LayerMapKind::Subcortical)
            .layer("external", 1, map::FF_OUT, LayerKind::Axonal(AxonKind::Spatial))
        )
        // .lmap(LayerMapScheme::new("motor_gen", LayerMapKind::Subcortical)
        //     .layer("whatever", 1, map::FF_OUT, LayerKind::Axonal(AxonKind::Spatial))
        // )
}


const DST_AREA_SCL: u32 = 8;
const AREA_SIDE: u32 = 16 * DST_AREA_SCL;


fn define_a_schemes() -> AreaSchemeList {
    // const ENCODE_SIZE: u32 = 32;

    AreaSchemeList::new()
        // .add_area(AreaScheme::new("m0", "motor_gen", AREA_SIDE))
        // .add_area(AreaScheme::irregular("v0", "v0_lm", [51, 271])
        .area(AreaScheme::irregular("v0", "v0_lm", [64, 64])
            .input(InputScheme::Custom)
        )
        .area(AreaScheme::new("v1", "v1_lm", AREA_SIDE)
            .eff_areas(vec!["v0"])
        )
}

// #########################
// ##### DISABLE STUFF #####
// #########################
#[allow(unused_mut)]
pub fn ca_settings() -> CorticalAreaSettings {
    let mut settings = CorticalAreaSettings::new();
    settings.bypass_inhib = true;
    settings.bypass_filters = true;
    settings.disable_pyrs = true;
    // settings.disable_ssts = true;
    // settings.disable_mcols = true;
    // settings.disable_regrowth = true;
    // settings.disable_learning = true;

    settings
}
