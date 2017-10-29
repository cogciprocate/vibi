//! Vibi with the config from config.rs in interactive mode.

extern crate time;
extern crate vibi;

use vibi::{window, config};
use vibi::bismit::{Cortex, Flywheel, Subcortex, InputGenerator};
// use vibi::bismit::subcortex::{};


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
        // let mut flywheel = Flywheel::from_blueprint(config::define_lm_schemes(),
        //     config::define_a_schemes(), Some(config::ca_settings()), command_rx, "v1");
        // let cortex = Cortex::builder(config::define_lm_schemes(), config::define_a_schemes())
        //     .ca_settings(config::ca_settings())
        //     .build().unwrap();
        let layer_map_schemes = config::define_lm_schemes();
        let area_schemes = config::define_a_schemes();

        let input_gen = InputGenerator::new(&layer_map_schemes[&area_schemes["v0"].layer_map_name()],
            &area_schemes["v0"]).unwrap();
        let subcortex = Subcortex::new().nucleus(input_gen);

        let cortex = Cortex::builder(layer_map_schemes, area_schemes)
            .ca_settings(config::ca_settings())
            .sub(subcortex)
            .build().unwrap();

        let mut flywheel = Flywheel::new(cortex, command_rx, "v1");

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


