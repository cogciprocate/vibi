macro_rules! printlny {
	// ($fmt:expr) => ( print!(concat!("\x1b[93m", $fmt, "\x1b[0m", "\n")) );
	// ($fmt:expr, $($arg:tt)*) => ( print!(concat!("\x1b[93m", $fmt, "\x1b[0m", "\n"), $($arg)*) );
	($fmt:expr) => ( print!(concat!(yellowify!($fmt), "\n")) );
	($fmt:expr, $($arg:tt)*) => ( print!(concat!(yellowify!($fmt), "\n"), $($arg)*) );
}


macro_rules! yellowify {
	($s:expr) => (concat!("\x1b[93m", $s, "\x1b[0m"));
}


// pub use self::input_czar::{ InputCzar, InputKind, InputSource };

pub mod visualize;
//pub mod autorun;
//mod synapse_drill_down;
// pub mod input_czar;
mod output_czar;
// mod motor_state;
//mod hybrid;
//mod renderer;

