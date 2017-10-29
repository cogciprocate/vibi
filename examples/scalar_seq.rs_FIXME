//! Encode a sequence of scalar values and display their representation.

#![allow(unused_imports)]

extern crate vibi;

use vibi::window;
use vibi::bismit::{Cortex, CorticalAreaSettings};
use vibi::bismit::map::{self, LayerTags, LayerMapKind, LayerMapScheme, LayerMapSchemeList,
    AreaScheme, AreaSchemeList, CellScheme, FilterScheme, InputScheme, AxonKind, LayerKind};
use vibi::bismit::flywheel::Flywheel;

fn main() {
    use std::thread;
    use std::sync::mpsc;

    let (command_tx, command_rx) = mpsc::channel();
    let (request_tx, request_rx) = mpsc::channel();
    let (response_tx, response_rx) = mpsc::channel();

    let th_flywheel = thread::Builder::new().name("flywheel".to_string()).spawn(move || {
        let mut flywheel = Flywheel::from_blueprint(define_lm_schemes(),
            define_a_schemes(), None, command_rx);
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
    // const OLFAC_UID: u32 = 102;

    LayerMapSchemeList::new()
        .lmap(LayerMapScheme::new("visual", LayerMapKind::Cortical)
            //.layer("test_noise", 1, map::DEFAULT, LayerKind::Axonal(Spatial))
            .axn_layer("motor_ctx", map::NS_IN | LayerTags::uid(MOTOR_UID), AxonKind::Horizontal)
            // .axn_layer("olfac", map::NS_IN | LayerTags::with_uid(OLFAC_UID), Horizontal)
            // .axn_layer("eff_in", map::FB_IN, AxonKind::Spatial)
            .axn_layer("aff_in", map::FF_IN, AxonKind::Spatial)
            // .axn_layer("out", map::FF_FB_OUT, Spatial)
            .axn_layer("unused", map::UNUSED_TESTING, AxonKind::Spatial)
            .layer("mcols", 1, map::FF_FB_OUT, CellScheme::minicolumn("iv", "iii"))
            .layer("iv_inhib", 0, map::DEFAULT, CellScheme::inhib(4, "iv"))

            .layer("iv", 1, map::PSAL,
                CellScheme::spiny_stellate(7, vec!["aff_in"], 400, 12))

            .layer("iii", 2, map::PTAL,
                CellScheme::pyramidal(1, 6, vec!["iii"], 500, 14)
                    // .apical(vec!["eff_in"/*, "olfac"*/], 18)
                )
        )
        .lmap(LayerMapScheme::new("v0_lm", LayerMapKind::Subcortical)
            .layer("external", 1, map::FF_OUT, LayerKind::Axonal(AxonKind::Spatial))
            // .layer("horiz_ns", 1, map::NS_OUT | LayerTags::uid(MOTOR_UID),
            //     LayerKind::Axonal(AxonKind::Horizontal))
        )
        // .lmap(LayerMapScheme::new("v0b_lm", LayerMapKind::Subcortical)
        //     .layer("spatial", 1, map::FF_OUT, LayerKind::Axonal(AxonKind::Spatial))
        //     // .layer("horiz_ns", 1, map::NS_OUT | LayerTags::uid(MOTOR_UID),
        //     //     LayerKind::Axonal(AxonKind::Horizontal))
        // )
}


fn define_a_schemes() -> AreaSchemeList {
    // const CYCLES_PER_FRAME: usize = 1;
    // const HZS: u32 = 16;
    const ENCODE_SIZE: u32 = 64; // had been used for GlyphSequences
    // const ENCODE_SIZE: u32 = 24; // for SensoryTract
    const AREA_SIDE: u32 = 48;

    AreaSchemeList::new()
        // .area_ext("v0", "v0_lm", ENCODE_SIZE,
        //     // InputScheme::GlyphSequences { seq_lens: (5, 5), seq_count: 10, scale: 1.4, hrz_dims: (16, 16) },
        //     InputScheme::ScalarSequence { range: (-8.0, 8.0), incr: 0.03 },
        //     None,
        //     None,
        // )
        // .area_ext("v0b", "v0b_lm", ENCODE_SIZE,
        //     InputScheme::SensoryTract,
        //     None,
        //     None,
        // )
        // .area("v1", "visual", AREA_SIDE,
        //     // Some(vec![FilterScheme::new("retina", None)]),
        //     None,
        //     Some(vec!["v0"]),
        //     // Some(vec!["v0b"]),
        // )
        .area(AreaScheme::new("v0", "v0_lm", ENCODE_SIZE)
            // .input(InputScheme::GlyphSequences { seq_lens: (5, 5), seq_count: 10, scale: 1.4, hrz_dims: (16, 16) }),
            .input(InputScheme::ScalarSequence { range: (-8.0, 8.0), incr: 0.03 }),
        )
        .area(AreaScheme::new("v1", "visual", AREA_SIDE)
            .eff_areas(vec!["v0"])
            .filter_chain(map::FF_IN, vec![FilterScheme::new("retina", None)])
        )
}

// #########################
// ##### DISABLE STUFF #####
// #########################
#[allow(unused_mut)]
pub fn ca_settings() -> CorticalAreaSettings {
    let mut settings = CorticalAreaSettings::new();

    // settings.bypass_inhib = true;
    // settings.bypass_filters = true;
    // settings.disable_pyrs = true;
    // settings.disable_ssts = true;
    // settings.disable_mcols = true;
    // settings.disable_regrowth = true;
    // settings.disable_learning = true;

    settings
}
