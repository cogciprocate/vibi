
#[macro_use] extern crate glium;
extern crate glium_text;
extern crate image;
extern crate time;
extern crate bismit;
extern crate find_folder;
extern crate num;
extern crate vecmath;

#[macro_use] extern crate conrod;
// extern crate piston_window;
// extern crate elmesque;
extern crate rustc_serialize;
// extern crate gfx_graphics;
// extern crate graphics;
// extern crate opengl_graphics;
// extern crate piston;

use std::thread;
use std::sync::mpsc;
// 
// use interactive::loop_cycles::{ self }; 
// use window_conrod as window;

#[macro_use] mod interactive;
mod config;
mod loop_cycles;
mod window;

// mod window_conrod;
// mod conrod_draw;
// mod widgets;


fn main() {
	#![allow(unused_variables)]
	println!("================= Bismit: vibi::main() running... =================");
	let time_start = time::get_time();	
	// tomfoolery(&time_start);

	
	let (status_tx, status_rx) = mpsc::channel();
	let (control_tx, control_rx) = mpsc::channel();

	let th_win = thread::Builder::new().name("win".to_string()).spawn(move || {
		window::window_main::open(control_tx, status_rx);
	}).expect("Error creating 'win' thread");

	// let th_vis = thread::Builder::new().name("vis".to_string()).spawn(move || {
	// 	loop_cycles::run(0, control_rx, status_tx);
	// }).expect("Error creating 'vis' thread");

	if let Err(e) = th_win.join() { println!("th_win.join(): Error: '{:?}'", e); }
	// if let Err(e) = th_vis.join() { println!("th_vin.join(): Error: '{:?}'", e); }


	// <<<<< MOVE THIS ELSEWHERE >>>>>
	let time_complete = time::get_time() - time_start;
	let t_sec = time_complete.num_seconds();
	let t_ms = time_complete.num_milliseconds() - (t_sec * 1000);
	println!("\n====== Bismit: vibi::main() complete in: {}.{} seconds ======", t_sec, t_ms);
}


