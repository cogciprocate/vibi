// use std::iter;
use std::io::{ self, Write };
use time::{ self, Timespec };

// use bismit::cmn::{ CorticalDims };
use bismit::map::{ self, LayerTags };
use bismit::cortex::{ self, Cortex };
// use bismit::encode:: { IdxReader };
use bismit::proto::{ ProtolayerMap, ProtolayerMaps, ProtoareaMaps, Axonal, Spatial, Horizontal, 
	Sensory, Thalamic, Protocell, Protofilter, Protoinput };
use bismit::input_source::{ InputGanglion };

use interactive::{ output_czar };


const INITIAL_TEST_ITERATIONS: i32 		= 1; 
const STATUS_EVERY: i32 				= 5000;
const PRINT_DETAILS_EVERY: i32			= 10000;

const CYCLES_PER_FRAME: usize 				= 1;

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
				Protocell::spiny_stellate(5, vec!["aff_in"], 700, 8))

			.layer("iii", 2, map::PTAL, 
				Protocell::pyramidal(1, 5, vec!["iii"], 800, 10)
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
	let area_side = 37 as u32;

	ProtoareaMaps::new()		
		//let mut ir_labels = IdxReader::new(CorticalDims::new(1, 1, 1, 0, None), "data/train-labels-idx1-ubyte", 1);
		// .area_ext("u0", "external", area_side, area_side, 
		// 	Protoinput::IdxReader { 
		// 		file_name: "data/train-labels-idx1-ubyte", 
		// 		cyc_per: CYCLES_PER_FRAME,
		// 	},

		// 	None, 
		// 	Some(vec!["u1"]),
		// )

		// .area("u1", "visual", area_side, area_side, None,
		// 	//None,
		// 	Some(vec!["b1"]),
		// )

		// .area_ext("o0sp", "v0_layer_map", area_side,
		// 	Protoinput::IdxReaderLoop { 
		// 		file_name: "data/train-images-idx3-ubyte", 
		// 		cyc_per: CYCLES_PER_FRAME, 
		// 		scale: 1.3,
		// 		loop_frames: 31,
		// 	},
		// 	None, 
		// 	None,
		// )

		// .area_ext("o0", "o0_lm", 24, Protoinput::Zeros, None, None)

		// .area("o1", "visual", area_side, 
		// 	None,
		// 	Some(vec!["o0sp", "o0nsp"]),
		// )

		.area_ext("v0", "v0_lm", area_side,
			Protoinput::IdxReaderLoop { 
				file_name: "data/train-images-idx3-ubyte", 
				cyc_per: CYCLES_PER_FRAME, 
				scale: 1.3,
				loop_frames: 11,
			},
			None, 
			None,
		)

		.area("v1", "visual", area_side, 
			Some(vec![Protofilter::new("retina", Some("filters.cl"))]),			
			Some(vec!["v0"/*, "o0"*/]),
		)

		.area("b1", "visual", area_side,
		 	None,		 	
		 	Some(vec!["v1"]),
		)


		// .area("a1", "visual", area_side, None, Some(vec!["b1"]))
		// .area("a2", "visual", area_side, None, Some(vec!["a1"]))
		// .area("a3", "visual", area_side, None, Some(vec!["a2"]))
		// .area("a4", "visual", area_side, None, Some(vec!["a3"]))
		// .area("a5", "visual", area_side, None, Some(vec!["a4"]))
		// .area("a6", "visual", area_side, None, Some(vec!["a5"]))
		// .area("a7", "visual", area_side, None, Some(vec!["a6"]))
		// .area("a8", "visual", area_side, None, Some(vec!["a7"]))
		// .area("a9", "visual", area_side, None, Some(vec!["a8"]))
		// .area("aA", "visual", area_side, None, Some(vec!["a9"]))
		// .area("aB", "visual", area_side, None, Some(vec!["aA"]))
		// .area("aC", "visual", area_side, None, Some(vec!["aB"]))
		// .area("aD", "visual", area_side, None, Some(vec!["aC"]))
		// .area("aE", "visual", area_side, None, Some(vec!["aD"]))
		// .area("aF", "visual", area_side, None, Some(vec!["aE"]))

}



/* RUN(): Run the interactive testing command line
	- TODO:
		- [incomplete][priority:very low] Proper command line using enums to 
		represent user input and a seperate struct to manage its state
			- Or just be lazy and leave it the beautiful disaster that it is...	
*/
pub fn run(autorun_iters: i32) -> bool {
	#![allow(unused_assignments, dead_code)]

	// KEEP UNTIL WE FIGURE OUT HOW TO GENERATE THIS NUMBER ELSEWHERE
	// let mut ir_labels_vec: Vec<u8> = Vec::with_capacity(1);
	// ir_labels_vec.push(0);

	// let mut ir_labels = IdxReader::new(CorticalDims::new(1, 1, 1, 0, None), 
	// 	"data/train-labels-idx1-ubyte", CYCLES_PER_FRAME, 1.0);
	
	let mut cortex = cortex::Cortex::new(define_plmaps(), define_pamaps());

	/* ************************* */
	/* ***** DISABLE STUFF ***** */	
	/* ************************* */
	for (_, area) in &mut cortex.areas {
		// area.psal_mut().dens_mut().syns_mut().set_offs_to_zero_temp();
		// area.bypass_inhib = true;
		// area.bypass_filters = true;
		// area.disable_pyrs = true;

		// area.disable_ssts = true;
		// area.disable_mcols = true;

		// area.disable_learning = true;
		// area.disable_regrowth = true;
		area.disable_regrowth = false;
	}
	
	// Unused right now (except for a .clear())
	// let mut input_status: String = String::with_capacity(100);

	let mut ri = RunInfo {
		cortex: cortex,
		test_iters: if autorun_iters > 0 {
				autorun_iters
			} else {
				INITIAL_TEST_ITERATIONS
			}, 
		bypass_act: false, 
		autorun_iters: autorun_iters, 
		first_run: true, 
		view_all_axons: false, 
		view_sdr_only: true,
		area_name: "v1".to_string(),
		cur_ttl_iters: 0i32,
		cur_cycle: 0i32,
		loop_start_time: time::get_time(),
	};

	loop {
		match prompt(&mut ri) {
			LoopAction::Continue => continue,
			LoopAction::Break => break,
			LoopAction::None => (),
		}

		ri.loop_start_time = time::get_time();
		ri.cur_cycle = 0;

		if ri.test_iters > 1 {
			print!("Running {} iterations... \n", ri.test_iters);
		}


		// Sense only loop:
		if !ri.view_sdr_only { print!("\nRunning {} sense only loop(s) ... \n", ri.test_iters - 1); }
		loop_cycles(&mut ri);


		// Sense and print loop:
		if !ri.view_sdr_only { print!("\n\nRunning {} sense and print loop(s)...", 1usize); }
		loop {
			if ri.cur_cycle >= (ri.test_iters) { break; }

			if !ri.bypass_act {
				// ir_labels.cycle(&mut ir_labels_vec[..]);
				ri.cortex.cycle();
				// input_status.clear();
				// let cur_frame = ri.cur_ttl_iters as usize % 5000;
				// input_status.push_str(&format!("[{}] -> '{}'", cur_frame, ir_labels_vec[0]));
			}

			if !ri.view_sdr_only {
				print!("\n\n=== Iteration {}/{} ===", ri.cur_cycle + 1, ri.test_iters);

				if false {
					print!("\nSENSORY INPUT VECTOR:");
				}

				output_czar::print_sense_and_print(&mut ri.cortex, &ri.area_name);
			}

			if ri.view_sdr_only { ri.cortex.area_mut(&ri.area_name).psal_mut().dens.states.read_wait(); }
			ri.cortex.area_mut(&ri.area_name).axns.states.read_wait();
			print!("\n'{}' output:", &ri.area_name);
			ri.cortex.area_mut(&ri.area_name).render_aff_out("", true);

			if ri.view_all_axons {
				print!("\n\nAXON SPACE:\n");
				
				ri.cortex.area_mut(&ri.area_name).render_axon_space();
			}			

			ri.cur_cycle += 1;

			if ri.cur_cycle > 1 {
				let t = time::get_time() - ri.loop_start_time;
				printlny!("-> {} cycles @ [> {:02.2} c/s <]", 
					ri.cur_cycle, (ri.cur_cycle as f32 / t.num_milliseconds() as f32) * 1000.0);
			}
		}

		if ri.test_iters > 1000 {
			ri.test_iters = 1;
		}

		if !ri.bypass_act {
			ri.cur_ttl_iters += ri.cur_cycle;
		}

		if ri.autorun_iters > 0 {
			break;
		}
	}

	println!("");

	true
}


fn loop_cycles(ri: &mut RunInfo) {
	loop {
		if ri.cur_cycle >= (ri.test_iters - 1) { break; }

		if ri.cur_cycle % STATUS_EVERY == 0 || ri.cur_cycle < 0 || ri.cur_cycle == (ri.test_iters - 2) {
			let t = time::get_time() - ri.loop_start_time;
			if ri.cur_cycle > 0 || (ri.test_iters > 1 && ri.cur_cycle == 0) {
				print!("[{}: {:01}ms]", ri.cur_cycle, t.num_milliseconds());
			}
			io::stdout().flush().ok();
		}

		if ri.cur_cycle % PRINT_DETAILS_EVERY == 0 || ri.cur_cycle < 0 {
			if !ri.view_sdr_only { 
				output_czar::print_sense_only(&mut ri.cortex, &ri.area_name); 
			}
		}
					
		if !ri.bypass_act {
			// ir_labels.cycle(&mut ir_labels_vec[..]);
			ri.cortex.cycle();
		}


		ri.cur_cycle += 1;
	}
}



fn prompt(ri: &mut RunInfo) -> LoopAction {
	if ri.test_iters == 0 {
		ri.test_iters = 1;
		ri.bypass_act = true; 
	} else {
		ri.bypass_act = false;
	}

	if ri.autorun_iters == 0 {
		let in_string: String = if ri.first_run {
			ri.first_run = false;
			"\n".to_string()
		} else {
			let axn_state = if ri.view_all_axons { "on" } else { "off" };
			let view_state = if ri.view_sdr_only { "sdr" } else { "all" };

			rin(format!("bismit: [{ttl_i}/({loop_i})]: [v]iew:[{}] [a]xons:[{}] \
				[m]otor:[X] a[r]ea:[{}] [t]ests [q]uit [i]ters:[{iters}]", 
				view_state, axn_state, ri.area_name, 
				iters = ri.test_iters,
				loop_i = 0, //input_czar.counter(), 
				ttl_i = ri.cur_ttl_iters,
			))
		};


		if "q\n" == in_string {
			print!("\nExiting interactive test mode... ");
			return LoopAction::Break;
		} else if "i\n" == in_string {
			let in_s = rin(format!("Iterations: [i={}]", ri.test_iters));
			if "\n" == in_s {
				return LoopAction::Continue;
			} else {
				let in_int: Option<i32> = parse_num(in_s);
				match in_int {
					Some(x)	=> {
						 ri.test_iters = x;
						 return LoopAction::None;
					},
					None    => {
						print!("Invalid number.\n");
						return LoopAction::Continue;
					},
				}
			}

		} else if "r\n" == in_string {
			let in_str = rin(format!("area name"));
			let in_s1 = in_str.trim();
			let new_area_name = in_s1.to_string();
			
			if ri.cortex.valid_area(&new_area_name) {
				ri.area_name = new_area_name;
			} else {
				print!("Invalid area.");
			}
			ri.bypass_act = true;
			return LoopAction::None;

		} else if "v\n" == in_string {
			ri.view_sdr_only = !ri.view_sdr_only;
			ri.bypass_act = true;
			return LoopAction::None;

		} else if "\n" == in_string {
			return LoopAction::None;
			// DO NOT REMOVE

		} else if "a\n" == in_string {
			ri.view_all_axons = !ri.view_all_axons;
			ri.bypass_act = true;
			return LoopAction::None;

		} else if "t\n" == in_string {
			let in_s = rin(format!("tests: [f]ract [c]ycles [l]earning [a]ctivate a[r]ea_output o[u]tput"));

			if "p\n" == in_s {
				//synapse_drill_down::print_pyrs(&mut cortex);
				return LoopAction::Continue;

			} else if "u\n" == in_s {
				// let in_str = rin(format!("area name"));
				// let in_s1 = in_str.trim();
				// let out_len = cortex.area(&in_s).dims.columns();
				// let t_vec: Vec<u8> = iter::repeat(0).take(out_len as usize).collect();
				// cortex.area_mut(&in_s).read_output(&mut t_vec, map::FF_OUT);
				// ocl::fmt::print_vec_simple(&t_vec);
				println!("\n##### PRINTING TEMPORARILY DISABLED #####");
				return LoopAction::Continue;

			} else if "c\n" == in_s {
				println!("\n##### DISABLED #####");
				//hybrid::test_cycles(&mut cortex, &area_name);
				return LoopAction::Continue;

			} else if "l\n" == in_s {
				println!("\n##### DISABLED #####");
				//learning::test_learning_cell_range(&mut cortex, inhib_layer_name, &area_name);
				return LoopAction::Continue;

			} else if "a\n" == in_s {
				println!("\n##### DISABLED #####");
				//learning::test_learning_activation(&mut cortex, &area_name);
				return LoopAction::Continue;

			// } else if "f\n" == in_s {
			// 	let in_s = rin(format!("fractal seed"));
			// 	let in_int: Option<u8> = in_s.trim().parse().ok();

			// 	// let seed = match in_int {
			// 	// 	Some(x)	=> x,
			// 	// 	None => {
			// 	// 		print!("\nError parsing number.");
			// 	// 		continue;
			// 	// 	},
			// 	// };

			// 	let in_s = rin(format!("cardinality factor: 256 * "));
			// 	let in_int: Option<usize> = in_s.trim().parse().ok();

			// 	let c_factor = match in_int {
			// 		Some(x)	=> x,
			// 		None => {
			// 			print!("\nError parsing number.");
			// 			continue;
			// 		},
			// 	};

			// 	// let tvec = cmn::gen_fract_sdr(seed, 256 * c_factor);
			// 	// ocl::fmt::print_vec_simple(&tvec[..]);
			// 	println!("\n##### PRINTING TEMPORARILY DISABLED #####");
			// 	continue;

			// } else if "r\n" == in_s {
			// 	let in_str = rin(format!("area name"));
			// 	// let in_s = in_str.trim();
			// 	//let in_int: Option<u8> = in_s.trim().parse().ok();

			// 	println!("\n##### DISABLED #####");
			// 	//cortex.print_area_output(&in_s);
			// 	continue;

			} else {
				return LoopAction::Continue;
			}


		} else if "m\n" == in_string {
			// bypass_act = true;
			let in_s = rin(format!("motor: [s]witch(disconnected)"));
			if "s\n" == in_s {
				//input_czar.motor_state.switch();
				//println!("\nREPLACE ME - synapse_sources::run() - line 100ish");
				return LoopAction::Continue;
				//test_iters = TEST_ITERATIONS;

			} else {
				return LoopAction::Continue;
			}
		} else {
			return LoopAction::Continue;
		}
	}

	LoopAction::None
}



pub fn rin(prompt: String) -> String {
	let mut in_string: String = String::new();
	print!("\n{}:> ", prompt);
	io::stdout().flush().unwrap();
	io::stdin().read_line(&mut in_string).ok().expect("Failed to read line");
	in_string.to_lowercase()
}

fn parse_num(in_s: String) -> Option<i32> {
	in_s.trim().replace("k","000").parse().ok()
	//in_s.trim().replace("m","000000").parse().ok()
}


struct RunInfo {
	cortex: Cortex,
	test_iters: i32, 
	bypass_act: bool, 
	autorun_iters: i32, 
	first_run: bool, 
	view_all_axons: bool, 
	view_sdr_only: bool,
	area_name: String,
	cur_ttl_iters: i32,
	cur_cycle: i32,
	loop_start_time: Timespec,
}

enum LoopAction {
	Break,
	Continue,
	None,
}
