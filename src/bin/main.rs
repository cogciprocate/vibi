//! Vibi with the config from config.rs in interactive mode.

extern crate time;
extern crate vibi;

use vibi::{cycle, window, config};


fn main() {
    use std::thread;
    use std::sync::mpsc;

    println!("================= Bismit: vibi::main() running... ==================");
    let time_start = time::get_time();
    // tomfoolery(&time_start);

    let (result_tx, result_rx) = mpsc::channel();
    let (control_tx, control_rx) = mpsc::channel();

    let th_win = thread::Builder::new().name("win".to_string()).spawn(move || {
        window::Window::open(control_tx, result_rx);
        // window::conrod::window_conrod::open(control_tx, result_rx);
    }).expect("Error creating 'win' thread");

    let th_vis = thread::Builder::new().name("vis".to_string()).spawn(move || {
        cycle::CycleLoop::run(0, control_rx, result_tx, config::define_lm_schemes(),
            config::define_a_schemes(), None);
    }).expect("Error creating 'vis' thread");

    if let Err(e) = th_win.join() { println!("th_win.join(): Error: '{:?}'", e); }
    if let Err(e) = th_vis.join() { println!("th_vin.join(): Error: '{:?}'", e); }


    // <<<<< MOVE THIS ELSEWHERE >>>>>
    let time_complete = time::get_time() - time_start;
    let t_sec = time_complete.num_seconds();
    let t_ms = time_complete.num_milliseconds() - (t_sec * 1000);
    println!("\n========= Bismit: vibi::main() complete in: {}.{} seconds =========", t_sec, t_ms);
}


