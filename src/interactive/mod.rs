macro_rules! printlny {
	// ($fmt:expr) => ( print!(concat!("\x1b[93m", $fmt, "\x1b[0m", "\n")) );
	// ($fmt:expr, $($arg:tt)*) => ( print!(concat!("\x1b[93m", $fmt, "\x1b[0m", "\n"), $($arg)*) );
	($fmt:expr) => ( print!(concat!(yellowify!($fmt), "\n")) );
	($fmt:expr, $($arg:tt)*) => ( print!(concat!(yellowify!($fmt), "\n"), $($arg)*) );
}


macro_rules! yellowify {
	($s:expr) => (concat!("\x1b[93m", $s, "\x1b[0m"));
}

use std::str::{ FromStr };
// pub use self::input_czar::{ InputCzar, InputKind, InputSource };

// pub mod cyc_loop;
//pub mod autorun;
//mod synapse_drill_down;
// pub mod input_czar;
pub mod output_czar;
// mod motor_state;
//mod hybrid;
//mod renderer;



pub fn parse_iters(in_s: &str) -> Result<u32, <u32 as FromStr>::Err> {
	in_s.trim().replace("k","000").parse()
	//in_s.trim().replace("m","000000").parse().ok()
}
