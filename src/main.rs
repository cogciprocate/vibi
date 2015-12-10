// #![feature(vec_push_all)]

extern crate time;
extern crate bismit;
#[macro_use] extern crate conrod;
extern crate find_folder;
extern crate piston_window;
extern crate elmesque;
extern crate num;
extern crate vecmath;
extern crate rustc_serialize;
#[macro_use] extern crate glium;
extern crate image;
// extern crate glutin_window;
// extern crate graphics;
// extern crate opengl_graphics;
// extern crate piston;

use std::thread;
use std::sync::mpsc;
use time::{ Timespec };
// use interactive::cyc_loop::{ self }; 

#[macro_use] mod interactive;
mod config;
mod cyc_loop;
// mod widgets;
// mod window;
mod window_main;
// mod teapot;
// type Ui = conrod::Ui<GlyphCache<'static>>;

fn main() {
	#![allow(unused_variables)]
	println!("================= Bismit: vibi::main() running... =================");
	let time_start = time::get_time();	
	// tomfoolery(&time_start);

	
	let (status_tx, status_rx) = mpsc::channel();
	let (control_tx, control_rx) = mpsc::channel();

	let th_win = thread::Builder::new().name("win".to_string()).spawn(move || {
		window_main::window(control_tx, status_rx);
	}).expect("Error creating 'win' thread");

	// let th_vis = thread::Builder::new().name("vis".to_string()).spawn(move || {
	// 	cyc_loop::run(0, control_rx, status_tx);
	// }).expect("Error creating 'vis' thread");

	if let Err(e) = th_win.join() { println!("th_win.join(): Error: '{:?}'", e); }
	// if let Err(e) = th_vis.join() { println!("th_vin.join(): Error: '{:?}'", e); }


	// <<<<< MOVE THIS ELSEWHERE >>>>>
	let time_complete = time::get_time() - time_start;
	let t_sec = time_complete.num_seconds();
	let t_ms = time_complete.num_milliseconds() - (t_sec * 1000);
	println!("\n====== Bismit: vibi::main() complete in: {}.{} seconds ======", t_sec, t_ms);
}





#[allow(dead_code)]
fn tomfoolery(ts: &Timespec) {
	use std::thread::{ self, JoinHandle };
	use std::sync::{ Arc, Mutex };
	use std::time::{ Duration };

	const TCOUNT: usize = 3;

	let mut handles = Vec::<JoinHandle<_>>::with_capacity(10);
	let data = Arc::new(Mutex::new(0));
	let (tx, rx) = mpsc::channel();

	for h_id in 0..TCOUNT {
		let (data, tx) = (data.clone(), tx.clone());

		handles.push(thread::Builder::new().name(h_id.to_string()).spawn(move || {

		// handles.push(thread::spawn(move || {
			let mut data = match data.lock() {
				Ok(d) => d,
				Err(err) => {
					println!("Error locking mutex: '{:?}'", err);
					tx.send((h_id, time::get_time(), "ERROR".to_string())).ok();
					return;
				},
			};

			*data += 1;            

			thread::sleep(Duration::new(0, h_id as u32 * 10000000));

			let msg = cyc_loop::rin(h_id.to_string());

			tx.send((h_id, time::get_time(), msg.clone())).ok();

			if msg.trim() == "" { panic!("Empty!"); }
		}).expect("Error creating thread"));
	}

	for i in 0..TCOUNT {
		let (t_id, tf, msg) = match rx.recv() {
			Ok(r) => r,
			Err(err) => {
				println!("Receive error: '{:?}'", err);
				return;
			},
		};

		let elpsd = (tf - *ts).num_milliseconds();

		println!("[msg: {}]: thread {} finished after {}ms with message: '{}'", i, t_id, elpsd, msg.trim());
	}

	print!("\n");

	for i in 0..TCOUNT {
		let rslt = match handles.pop() {
			Some(r) => r.join(),
			None => {
				println!("Thread {} failed.", i);
				break;
			}
		};

		println!("Result of thread {} is '{:?}'", i, rslt);
	}

	println!("\n{:?}", data);
}



// fn ms_since(start: &Timespec) -> i64 {
// 	let time_elapsed = time::get_time() - *start;
// 	time_elapsed.num_milliseconds()
// }
