#[macro_use] extern crate glium;
extern crate glium_text;
extern crate image;
extern crate time;
extern crate bismit;
extern crate find_folder;
extern crate num;
extern crate vecmath;
extern crate rustc_serialize;
extern crate rand;
// #[macro_use] extern crate conrod;
// extern crate piston_window;
// extern crate elmesque;
// extern crate gfx_graphics;
// extern crate graphics;
// extern crate opengl_graphics;
// extern crate piston;

// 
// use interactive::loop_cycles::{ self };
// use window_conrod as window;

#[macro_use] mod interactive;
mod config;
mod window;
mod util;

// mod conrod_draw;
// mod widgets;


fn main() {
    #![allow(unused_variables)]
    // use std::iter;
    use std::thread;
    use std::sync::mpsc;
    // use ganglion_buffer::TractBuffer;
    
    println!("================= Bismit: vibi::main() running... ==================");
    let time_start = time::get_time();    
    // tomfoolery(&time_start);

    let (result_tx, result_rx) = mpsc::channel();
    let (control_tx, control_rx) = mpsc::channel();

    let th_win = thread::Builder::new().name("win".to_string()).spawn(move || {
        window::MainWindow::open(control_tx, result_rx);
        // window::conrod::window_conrod::open(control_tx, result_rx);
    }).expect("Error creating 'win' thread");

    let th_vis = thread::Builder::new().name("vis".to_string()).spawn(move || {
        interactive::CycleLoop::run(0, control_rx, result_tx);
    }).expect("Error creating 'vis' thread");

    if let Err(e) = th_win.join() { println!("th_win.join(): Error: '{:?}'", e); }
    if let Err(e) = th_vis.join() { println!("th_vin.join(): Error: '{:?}'", e); }


    // <<<<< MOVE THIS ELSEWHERE >>>>>
    let time_complete = time::get_time() - time_start;
    let t_sec = time_complete.num_seconds();
    let t_ms = time_complete.num_milliseconds() - (t_sec * 1000);
    println!("\n========= Bismit: vibi::main() complete in: {}.{} seconds =========", t_sec, t_ms);
}


