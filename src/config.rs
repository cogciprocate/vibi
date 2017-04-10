//! Default configuration for vibi used when run as binary.

// use find_folder::Search;
use bismit::Cortex;
use bismit::map::{self, LayerMapKind, LayerMapScheme, LayerMapSchemeList,
    AreaScheme, AreaSchemeList, CellScheme, InputScheme, AxonTopology, LayerKind,
    AxonDomain, InputTrack};
use bismit::encode::GlyphSequences;
// use bismit::proto::{ProtolayerMap, ProtolayerMaps, ProtoareaMaps, Axonal, Spatial, Horizontal,
//     Cortical, Thalamic, Protocell, Protofilter, Protoinput};

/* Eventually move defines to a config file or some such */
pub fn define_lm_schemes() -> LayerMapSchemeList {
    // const MOTOR_UID: u32 = 543;
    // const OLFAC_UID: u32 = 654;

    LayerMapSchemeList::new()
        .lmap(LayerMapScheme::new("v1_lm", LayerMapKind::Cortical)
            //.layer("test_noise", 1, map::DEFAULT, LayerKind::Axonal(Spatial))

            // .input_layer("motor_ctx", map::NS_IN | LayerTags::uid(MOTOR_UID),
            //     AxonDomain::Local, AxonTopology::Horizontal)
            .input_layer("motor_ctx", map::DEFAULT,
                AxonDomain::input(&[(InputTrack::Afferent, GlyphSequences::val_lyr_tags())]),
                AxonTopology::Horizontal
            )

            // .input_layer("olfac", map::NS_IN | LayerTags::with_uid(OLFAC_UID), Horizontal)
            // .input_layer("eff_in", map::FB_IN | LayerTags::uid(U0),
            //     AxonDomain::input(&[(InputTrack::Efferent, &[map::THAL_SP])]),
            //     AxonTopology::Spatial
            // )

            .input_layer("aff_in", map::DEFAULT,
                AxonDomain::input(&[(InputTrack::Afferent, GlyphSequences::img_lyr_tags())]),
                AxonTopology::Spatial
            )
            .input_layer("unused", map::UNUSED_TESTING, AxonDomain::Local, AxonTopology::Spatial)
            .layer("mcols", 1, map::DEFAULT, AxonDomain::output(&[map::THAL_SP]),
                CellScheme::minicolumn("iv", "iii")
            )
            .layer("iv", 1, map::PSAL, AxonDomain::Local,
                CellScheme::spiny_stellate(&[("aff_in", 12, 1)], 4, 400)
            )
            .layer("iv_inhib", 0, map::DEFAULT, AxonDomain::Local, CellScheme::inhibitory(4, "iv"))
            .layer("iii", 2, map::PTAL, AxonDomain::Local,
                CellScheme::pyramidal(&[("iii", 20, 1)], 2, 5, 800)
                    // .apical(&[("eff_in", 22)], 1, 5, 500)
            )
        )
        .lmap(LayerMapScheme::new("v0_lm", LayerMapKind::Subcortical)
            .layer("horiz_ns", 1, map::DEFAULT,
                AxonDomain::output(GlyphSequences::val_lyr_tags()),
                LayerKind::Axonal(AxonTopology::Horizontal))

            .layer("spatial", 1, map::DEFAULT,
                AxonDomain::output(GlyphSequences::img_lyr_tags()),
                LayerKind::Axonal(AxonTopology::Spatial))
        )
}


pub fn define_a_schemes() -> AreaSchemeList {
    // const CYCLES_PER_FRAME: usize = 1;
    // const HZS: u32 = 16;
    const ENCODE_SIZE: u32 = 32; // had been used for GlyphSequences
    // const ENCODE_SIZE: u32 = 24; // for SensoryTract
    const AREA_SIDE: u32 = 48;

    AreaSchemeList::new()
        // .area_ext("v0", "v0_lm", ENCODE_SIZE,
        //     InputScheme::GlyphSequences { seq_lens: (5, 5), seq_count: 10, scale: 1.4, hrz_dims: (16, 16) },
        //     None,
        //     None,
        // )
        .area(AreaScheme::new("v0", "v0_lm", ENCODE_SIZE)
            .input(InputScheme::GlyphSequences { seq_lens: (5, 5), seq_count: 10,
                scale: 1.4, hrz_dims: (16, 16) })
        )

        // .area_ext("v0b", "v0b_lm", ENCODE_SIZE,
        //     InputScheme::SensoryTract,
        //     None,
        //     None,
        // )
        // .area("v1", "visual", AREA_SIDE,
        //     Some(vec![FilterScheme::new("retina", None)]),
        //     Some(vec!["v0"]),
        //     // Some(vec!["v0b"]),
        // )
        .area(AreaScheme::new("v1", "v1_lm", AREA_SIDE)
            .eff_areas(vec!["v0"])
            .filter_chain(InputTrack::Afferent, GlyphSequences::img_lyr_tags(),
                &[("retina", None)])
        )

        // .area("b1", "visual", AREA_SIDE,
        //      None,
        //      Some(vec!["v1"]),
        // )


        // .area("a1", "visual", AREA_SIDE, None, Some(vec!["b1"]))
        // .area("a2", "visual", AREA_SIDE, None, Some(vec!["a1"]))
        // .area("a3", "visual", AREA_SIDE, None, Some(vec!["a2"]))
        // .area("a4", "visual", AREA_SIDE, None, Some(vec!["a3"]))
        // .area("a5", "visual", AREA_SIDE, None, Some(vec!["a4"]))
        // .area("a6", "visual", AREA_SIDE, None, Some(vec!["a5"]))
        // .area("a7", "visual", AREA_SIDE, None, Some(vec!["a6"]))
        // .area("a8", "visual", AREA_SIDE, None, Some(vec!["a7"]))
        // .area("a9", "visual", AREA_SIDE, None, Some(vec!["a8"]))
        // .area("aA", "visual", AREA_SIDE, None, Some(vec!["a9"]))
        // .area("aB", "visual", AREA_SIDE, None, Some(vec!["aA"]))
        // .area("aC", "visual", AREA_SIDE, None, Some(vec!["aB"]))
        // .area("aD", "visual", AREA_SIDE, None, Some(vec!["aC"]))
        // .area("aE", "visual", AREA_SIDE, None, Some(vec!["aD"]))
        // .area("aF", "visual", AREA_SIDE, None, Some(vec!["aE"]))


        //let mut ir_labels = IdxStreamer::new(LayerMapKind::CorticalDims::new(1, 1, 1, 0, None), "data/train-labels-idx1-ubyte", 1);
        // .area_ext("u0", "external", AREA_SIDE, AREA_SIDE,
        //     InputScheme::IdxStreamer {
        //         file_name: "data/train-labels-idx1-ubyte",
        //         cyc_per: CYCLES_PER_FRAME,
        //     },

        //     None,
        //     Some(vec!["u1"]),
        // )

        // .area("u1", "visual", AREA_SIDE, AREA_SIDE, None,
        //     //None,
        //     Some(vec!["b1"]),
        // )

        // .area_ext("o0sp", "v0_layer_map", AREA_SIDE,
        //     InputScheme::IdxStreamerLoop {
        //         file_name: "data/train-images-idx3-ubyte",
        //         cyc_per: CYCLES_PER_FRAME,
        //         scale: 1.3,
        //         loop_frames: 31,
        //     },
        //     None,
        //     None,
        // )

        // .area_ext("o0", "o0_lm", 24, InputScheme::Zeros, None, None)

        // .area("o1", "visual", AREA_SIDE,
        //     None,
        //     Some(vec!["o0sp", "o0nsp"]),
        // )

}

#[allow(unused_variables)]
pub fn disable_stuff(cortex: &mut Cortex) {

    /* ######################### */
    /* ##### DISABLE STUFF ##### */
    /* ######################### */
    // for (_, area) in &mut cortex.areas {
    //     // area.psal_mut().dens_mut().syns_mut().set_offs_to_zero_temp();
    //     // area.bypass_inhib = true;
    //     // area.bypass_filters = true;
    //     // area.disable_pyrs = true;

    //     // area.disable_ssts = true;
    //     // area.disable_mcols = true;

    //     // area.disable_learning = true;
    //     // area.disable_regrowth = true;
    // }
}
