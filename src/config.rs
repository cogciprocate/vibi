// use find_folder::Search;
use bismit::cortex::{Cortex};
use bismit::map::{self, LayerTags};
use bismit::proto::{ProtolayerMap, ProtolayerMaps, ProtoareaMaps, Axonal, Spatial, Horizontal, 
    Sensory, Thalamic, Protocell, Protofilter, Protoinput};

/* Eventually move defines to a config file or some such */
pub fn define_plmaps() -> ProtolayerMaps {
    // const MOTOR_UID: u32 = 543;
    const OLFAC_UID: u32 = 654;

    ProtolayerMaps::new()
        .lmap(ProtolayerMap::new("visual", Sensory)
            //.layer("test_noise", 1, map::DEFAULT, Axonal(Spatial))
            // .axn_layer("motor_in", map::NS_IN | LayerTags::with_uid(MOTOR), Horizontal)
            .axn_layer("olfac", map::NS_IN | LayerTags::with_uid(OLFAC_UID), Horizontal)
            .axn_layer("eff_in", map::FB_IN, Spatial)
            .axn_layer("aff_in", map::FF_IN, Spatial)
            // .axn_layer("out", map::FF_FB_OUT, Spatial)
            .axn_layer("unused", map::UNUSED_TESTING, Spatial)
            .layer("mcols", 1, map::FF_FB_OUT, Protocell::minicolumn("iv", "iii"))
            .layer("iv_inhib", 0, map::DEFAULT, Protocell::inhibitory(4, "iv"))

            .layer("iv", 1, map::PSAL, 
                Protocell::spiny_stellate(4, vec!["aff_in"], 400, 8))

            .layer("iii", 2, map::PTAL, 
                Protocell::pyramidal(1, 4, vec!["iii"], 800, 10)
                    .apical(vec!["eff_in"/*, "olfac"*/], 12))
        )

        .lmap(ProtolayerMap::new("v0_lm", Thalamic)
            .layer("ganglion", 1, map::FF_OUT, Axonal(Spatial))
        )

        .lmap(ProtolayerMap::new("o0_lm", Thalamic)
            .layer("ganglion", 1, map::NS_OUT | LayerTags::with_uid(OLFAC_UID), Axonal(Horizontal))
        )
}


pub fn define_pamaps() -> ProtoareaMaps {
    const AREA_SIDE: u32 = 16;
    // const CYCLES_PER_FRAME: usize = 1;

    // let file_name = Search::ParentsThenKids(3, 3).for_folder("data").unwrap().join("kernel_file.cl");

    ProtoareaMaps::new()        
        //let mut ir_labels = IdxStreamer::new(CorticalDims::new(1, 1, 1, 0, None), "data/train-labels-idx1-ubyte", 1);
        // .area_ext("u0", "external", AREA_SIDE, AREA_SIDE, 
        //     Protoinput::IdxStreamer { 
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
        //     Protoinput::IdxStreamerLoop { 
        //         file_name: "data/train-images-idx3-ubyte", 
        //         cyc_per: CYCLES_PER_FRAME, 
        //         scale: 1.3,
        //         loop_frames: 31,
        //     },
        //     None, 
        //     None,
        // )

        // .area_ext("o0", "o0_lm", 24, Protoinput::Zeros, None, None)

        // .area("o1", "visual", AREA_SIDE, 
        //     None,
        //     Some(vec!["o0sp", "o0nsp"]),
        // )

        .area_ext("v0", "v0_lm", AREA_SIDE,
            // Protoinput::IdxStreamerLoop { 
            //     file_name: "data/train-images-idx3-ubyte".to_string(), 
            //     cyc_per: CYCLES_PER_FRAME, 
            //     scale: 1.4,
            //     loop_frames: 80,
            // },
            Protoinput::GlyphSequences { seq_lens: (5, 5), seq_count: 10, scale: 1.4 },
            None, 
            None,
        )

        .area("v1", "visual", AREA_SIDE, 
            Some(vec![Protofilter::new("retina", Some("filters.cl"))]),            
            Some(vec!["v0"/*, "o0"*/]),
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

}

#[allow(unused_variables)]
pub fn disable_stuff(cortex: &mut Cortex) {

    /* ######################### */
    /* ##### DISABLE STUFF ##### */    
    /* ######################### */
    for (_, area) in &mut cortex.areas {
        // area.psal_mut().dens_mut().syns_mut().set_offs_to_zero_temp();
        // area.bypass_inhib = true;
        // area.bypass_filters = true;
        // area.disable_pyrs = true;

        // area.disable_ssts = true;
        // area.disable_mcols = true;

        // area.disable_learning = true;
        // area.disable_regrowth = true;
    }
}
