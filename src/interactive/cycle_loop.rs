use std::io::{self, Write};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{Arc, Mutex};
use time::{self, Timespec, Duration};

use bismit::cortex::{self, Cortex};
use bismit::input_source::{InputGanglion};
use config;

use interactive::{self, output_czar, CyCtl, CyRes, CyStatus};

const INITIAL_TEST_ITERATIONS: u32 	= 1; 
const STATUS_EVERY: u32 			= 5000;
const PRINT_DETAILS_EVERY: u32		= 10000;
const GUI_CONTROL: bool				= true;

pub struct CycleLoop;

impl CycleLoop {
	pub fn run(autorun_iters: u32, control_rx: Receiver<CyCtl>, mut result_tx: Sender<CyRes>)
				-> bool 
	{
		let mut cortex = cortex::Cortex::new(config::define_plmaps(), config::define_pamaps());
		config::disable_stuff(&mut cortex);

		let area_name = "v1".to_string();
		
		// let area_dims = { 
		// 	let dims = cortex.area(&area_name).dims();
		// 	(dims.v_size(), dims.u_size())
		// };
		
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
			area_name: area_name,
			status: CyStatus::new(/*area_dims*/),
			loop_start_time: time::get_time(),
		};

		// result_tx.send(CyRes::Status(ri.status.clone())).expect("Error sending initial status.");

		loop {
			if GUI_CONTROL {
				match control_rx.recv() {
					Ok(cyctl) => match cyctl {
						CyCtl::Iterate(i) => ri.test_iters = i,
						CyCtl::Exit => break,
						CyCtl::Sample(buf) => {
							refresh_gang_buf(&ri, buf);
							continue;
						},
						CyCtl::RequestCurrentAreaInfo => {
							result_tx.send(CyRes::CurrentAreaInfo(
									ri.area_name.to_string(),
									ri.cortex.area(&ri.area_name).area_map().aff_out_slc_range(),
									ri.cortex.area(&ri.area_name).axn_gang_map()
								)).expect("Error sending area name.");
							continue;
						},
						_ => continue,
					},

					Err(e) => panic!("run(): control_rx.recv(): '{:?}'", e),
				}
			} else {
				match prompt(&mut ri) {
					LoopAction::Continue => continue,
					LoopAction::Break => break,
					LoopAction::None => (),
				}
			}		

			ri.loop_start_time = time::get_time();
			ri.status.cur_cycle = 0;
			ri.status.cur_elapsed = Duration::seconds(0);

			if ri.test_iters > 1 {
				print!("Running {} iterations... \n", ri.test_iters);
			}

			match loop_cycles(&mut ri, &control_rx, &mut result_tx) {
				CyCtl::Exit => break,
				_ => (),
			}

			match cycle_print(&mut ri) {
				LoopAction::Continue => continue,
				LoopAction::Break => break,
				LoopAction::None => (),
			}

			ri.status.ttl_cycles += ri.status.cur_cycle;
			ri.status.ttl_elapsed = ri.status.ttl_elapsed + ri.status.cur_elapsed;
			ri.status.cur_cycle = 0;
			ri.status.cur_elapsed = Duration::seconds(0);
			result_tx.send(CyRes::Status(ri.status.clone())).ok();
		}

		println!("");

		true
	}
}

fn refresh_gang_buf(ri: &RunInfo, buf: Arc<Mutex<Vec<u8>>>) {
	match buf.lock() {
		Ok(ref mut b) => ri.cortex.area(&ri.area_name).sample_aff_out(b),
		Err(e) => panic!("Error locking ganglion buffer mutex: {:?}", e),
	}
}


fn loop_cycles(ri: &mut RunInfo, control_rx: &Receiver<CyCtl>, result_tx: &mut Sender<CyRes>)
		-> CyCtl
{
	if !ri.view_sdr_only { print!("\nRunning {} sense only loop(s) ... \n", ri.test_iters - 1); }
	
	loop {
		if ri.status.cur_cycle >= (ri.test_iters - 1) { break; }		

		let t = time::get_time() - ri.loop_start_time;

		if ri.status.cur_cycle % STATUS_EVERY == 0 || ri.status.cur_cycle == (ri.test_iters - 2) {			
			if ri.status.cur_cycle > 0 || (ri.test_iters > 1 && ri.status.cur_cycle == 0) {
				print!("[{}: {:01}ms]", ri.status.cur_cycle, t.num_milliseconds());
			}
			io::stdout().flush().ok();
		}

		if ri.status.cur_cycle % PRINT_DETAILS_EVERY == 0 {
			if !ri.view_sdr_only { 
				output_czar::print_sense_only(&mut ri.cortex, &ri.area_name); 
			}
		}
					
		if !ri.bypass_act {
			ri.cortex.cycle();
		}

		// Check if any controls requests have been sent:
		if let Ok(c) = control_rx.try_recv() {
			match c {
				// If a new sample has been requested, fulfill it:
				CyCtl::Sample(buf) => refresh_gang_buf(&ri, buf),
				// Otherwise return with the control code:
				_ => return c,
			}
		}

		// Update and send status:
		ri.status.cur_cycle += 1;
		ri.status.cur_elapsed = t;
		result_tx.send(CyRes::Status(ri.status.clone())).ok();
	}

	CyCtl::None
}


fn cycle_print(ri: &mut RunInfo) -> LoopAction {
	if !ri.view_sdr_only { print!("\n\nRunning {} sense and print loop(s)...", 1usize); }

	if !ri.bypass_act {
		ri.cortex.cycle();
		ri.status.cur_cycle += 1;
	}

	if !ri.view_sdr_only {
		print!("\n\n=== Iteration {}/{} ===", ri.status.cur_cycle, ri.test_iters);

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
		
		ri.cortex.area_mut(&ri.area_name).render_axn_space();
	}	

	if ri.status.cur_cycle > 1 {
		printlny!("-> {} cycles @ [> {:02.2} c/s <]", 
			ri.status.cur_cycle, (ri.status.cur_cycle as f32 
				/ ri.status.cur_elapsed.num_milliseconds() as f32) * 1000.0);
	}	

	if ri.test_iters > 1000 {
		ri.test_iters = 1;
	}

	if ri.autorun_iters > 0 {
		LoopAction::Break
	} else {
		LoopAction::None
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
				ttl_i = ri.status.ttl_cycles,
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
				let in_int = interactive::parse_iters(&in_s);
				match in_int {
					Ok(x)	=> {
						 ri.test_iters = x;
						 return LoopAction::None;
					},
					Err(_) => {
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


struct RunInfo {
	cortex: Cortex,
	test_iters: u32, 
	bypass_act: bool, 
	autorun_iters: u32, 
	first_run: bool, 
	view_all_axons: bool, 
	view_sdr_only: bool,
	area_name: String,
	status: CyStatus,	
	loop_start_time: Timespec,
}

pub enum LoopAction {
	None,
	Break,
	Continue,
}
