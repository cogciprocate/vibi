//! Vibi with the config from config.rs in interactive mode.

extern crate time;
extern crate vibi;

use vibi::{window, config};
use vibi::bismit::flywheel::Flywheel;


fn main() {
    use std::thread;
    use std::sync::mpsc;

    println!("================= Bismit: vibi::main() running... ==================");
    let time_start = time::get_time();
    // tomfoolery(&time_start);

    let (command_tx, command_rx) = mpsc::channel();
    let (request_tx, request_rx) = mpsc::channel();
    let (response_tx, response_rx) = mpsc::channel();

    let th_flywheel = thread::Builder::new().name("flywheel".to_string()).spawn(move || {
        let mut flywheel = Flywheel::from_blueprint(command_rx, config::define_lm_schemes(),
            config::define_a_schemes(), None);
        flywheel.add_req_res_pair(request_rx, response_tx);
        flywheel.spin();
    }).expect("Error creating 'flywheel' thread");

    let th_win = thread::Builder::new().name("win".to_string()).spawn(move || {
        window::Window::open(command_tx, request_tx, response_rx);
    }).expect("Error creating 'win' thread");

    if let Err(e) = th_win.join() { println!("th_win.join(): Error: '{:?}'", e); }
    if let Err(e) = th_flywheel.join() { println!("th_vin.join(): Error: '{:?}'", e); }


    // <<<<< MOVE THIS ELSEWHERE >>>>>
    let time_complete = time::get_time() - time_start;
    let t_sec = time_complete.num_seconds();
    let t_ms = time_complete.num_milliseconds() - (t_sec * 1000);
    println!("\n========= Bismit: vibi::main() complete in: {}.{} seconds =========", t_sec, t_ms);
}


