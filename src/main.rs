use time::{ Timespec };

use interactive::visualize;

extern crate time;
extern crate bismit;

#[macro_use]
mod interactive;

fn main() {
	println!("================= Bismit: visbis::main() running... =================");
	let time_start = time::get_time();

	// if true {
	// 	interactive::visualize::run(0);
	// } else {
	// 	for i in 0..20 {
	// 		interactive::visualize::run(7000);
	// 	}
	// }
	
	interactive::visualize::run(0);

	// tomfoolery(&time_start);


	// <<<<< MOVE THIS ELSEWHERE >>>>>
	let time_complete = time::get_time() - time_start;
	let t_sec = time_complete.num_seconds();
	let t_ms = time_complete.num_milliseconds() - (t_sec * 1000);
	println!("\n====== Bismit: visbis::main() complete in: {}.{} seconds ======", t_sec, t_ms);
}


#[allow(dead_code)]
fn tomfoolery(ts: &Timespec) {
	use std::thread::{ self, JoinHandle };
	use std::sync::{ Arc, Mutex };
	use std::sync::mpsc;
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

            let msg = visualize::rin(h_id.to_string());

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
