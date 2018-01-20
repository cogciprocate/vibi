//! Default configuration for vibi used when run as binary.

use bismit::CorticalAreaSettings;
use bismit::map::*;
use bismit::encode::GlyphSequences;


/* Eventually move defines to a config file or some such */
pub fn define_lm_schemes() -> LayerMapSchemeList {
    // const MOTOR_UID: u32 = 543;
    // const OLFAC_UID: u32 = 654;

    LayerMapSchemeList::new()
        .lmap(LayerMapScheme::new("v1_lm", LayerMapKind::Cortical)
            .layer(LayerScheme::define("motor_ctx")
                .axonal(AxonTopology::Nonspatial)
                .axon_domain(AxonDomain::input(&[(InputTrack::Afferent,
                    GlyphSequences::val_lyr_tags())]))
            )
            .layer(LayerScheme::define("aff_in")
                .axonal(AxonTopology::Spatial)
                .axon_domain(AxonDomain::input(&[(InputTrack::Afferent,
                    GlyphSequences::img_lyr_tags())]))
            )
            .layer(LayerScheme::define("unused")
                .depth(1)
                .tags(LayerTags::UNUSED)
                .axonal(AxonTopology::Spatial)
                .axon_domain(AxonDomain::Local)
            )
            .layer(LayerScheme::define("iv")
                .depth(1)
                .tags(LayerTags::PSAL)
                .axon_domain(AxonDomain::Local)
                .cellular(CellScheme::spiny_stellate()
                    .tft(TuftScheme::basal().proximal()
                        .syns_per_den_l2(4)
                        .thresh_init(400)
                        .src_lyr(TuftSourceLayer::define("aff_in")
                            .syn_reach(12)
                            .prevalence(1)
                        )
                    )
                )
            )
            .layer(LayerScheme::define("iv_inhib")
                .cellular(CellScheme::control(
                        ControlCellKind::InhibitoryBasketSurround {
                            host_lyr_name: "iv".into(),
                            field_radius: 4,
                        },
                        0
                    )
                )
            )
            .layer(LayerScheme::define("iii")
                .depth(3)
                .tags(LayerTags::PTAL)
                .axon_domain(AxonDomain::output(&[AxonTag::unique()]))
                .cellular(CellScheme::pyramidal()
                    // .tft(TuftScheme::basal().proximal()
                    //     .syns_per_den_l2(0)
                    //     .src_lyr(TuftSourceLayer::define("aff_in_0")
                    //         .syn_reach(0)
                    //         .prevalence(1)
                    //     )
                    // )
                    .tft(TuftScheme::basal().distal()
                        .dens_per_tft_l2(2)
                        .syns_per_den_l2(5)
                        .max_active_dens_l2(2)
                        .thresh_init(800)
                        .src_lyr(TuftSourceLayer::define("iii")
                            .syn_reach(10)
                            .prevalence(1)
                        )
                    )
                    // .tft(TuftScheme::apical().distal()
                    //     .dens_per_tft_l2(1)
                    //     .syns_per_den_l2(5)
                    //     .max_active_dens_l2(0)
                    //     .thresh_init(500)
                    //     .src_lyr(TuftSourceLayer::define("iii")
                    //         .syn_reach(3)
                    //         .prevalence(1)
                    //     )
                    // )
                )
            )
            .layer(LayerScheme::define("iii_output")
                .cellular(CellScheme::control(
                        ControlCellKind::PyrOutputter {
                            host_lyr_name: "iii".into(),
                        },
                        0
                    )
                )
            )
        )
        .lmap(LayerMapScheme::new("v0_lm", LayerMapKind::Subcortical)
            .layer(LayerScheme::define("horiz_ns")
                .depth(1)
                .axonal(AxonTopology::Nonspatial)
                .axon_domain(AxonDomain::output(GlyphSequences::val_lyr_tags()))
            )
            .layer(LayerScheme::define("spatial")
                .depth(1)
                .axonal(AxonTopology::Spatial)
                .axon_domain(AxonDomain::output(GlyphSequences::img_lyr_tags()))
            )
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
        //     EncoderScheme::GlyphSequences { seq_lens: (5, 5), seq_count: 10,
        //    scale: 1.4, hrz_dims: (16, 16) },
        //     None,
        //     None,
        // )
        .area(AreaScheme::new("v0", "v0_lm", ENCODE_SIZE)
            .encoder(EncoderScheme::GlyphSequences { seq_lens: (5, 5), seq_count: 10,
                scale: 1.4, hrz_dims: (16, 16) })
        )

        // .area_ext("v0b", "v0b_lm", ENCODE_SIZE,
        //     EncoderScheme::SensoryTract,
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
        //     EncoderScheme::IdxStreamer {
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
        //     EncoderScheme::IdxStreamerLoop {
        //         file_name: "data/train-images-idx3-ubyte",
        //         cyc_per: CYCLES_PER_FRAME,
        //         scale: 1.3,
        //         loop_frames: 31,
        //     },
        //     None,
        //     None,
        // )

        // .area_ext("o0", "o0_lm", 24, EncoderScheme::Zeros, None, None)

        // .area("o1", "visual", AREA_SIDE,
        //     None,
        //     Some(vec!["o0sp", "o0nsp"]),
        // )

}

pub fn ca_settings() -> CorticalAreaSettings {
    #[allow(unused_imports)]
    use bismit::ocl::builders::BuildOpt;

    CorticalAreaSettings::new()
        // .bypass_inhib()
        // .bypass_filters()
        // .disable_pyrs()
        // .disable_ssts()
        // .disable_mcols()
        // .disable_regrowth()
        // .disable_learning()
        // .build_opt(BuildOpt::cmplr_def("DEBUG_SMOOTHER_OVERLAP", 1))
}