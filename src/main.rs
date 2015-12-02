extern crate time;
extern crate bismit;

#[macro_use]
mod interactive;

fn main() {
	println!("================= Bismit: visbis::main() running... =================");
	let time_start = time::get_time();
	// test_1::run_kernel();
	// sense::ascii_sense();
	// test_3::run();
	// test_casting::run();
	// hello_world::run();

	// if true {
	// 	interactive::visualize::run(0);
	// } else {
	// 	for i in 0..20 {
	// 		interactive::visualize::run(7000);
	// 	}
	// }
	
	//test_miccos::run();
	//test_reader();

	interactive::visualize::run(0);	

	// <<<<< MOVE THIS TO CMN AND MAKE A FUNCTION FOR IT >>>>>
	let time_complete = time::get_time() - time_start;
	let t_sec = time_complete.num_seconds();
	let t_ms = time_complete.num_milliseconds() - (t_sec * 1000);
	println!("\n====== Bismit: visbis::main() complete in: {}.{} seconds ======", t_sec, t_ms);
}
