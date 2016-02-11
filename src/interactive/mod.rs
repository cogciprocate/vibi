macro_rules! printlny {
    // ($fmt:expr) => ( print!(concat!("\x1b[93m", $fmt, "\x1b[0m", "\n")) );
    // ($fmt:expr, $($arg:tt)*) => ( print!(concat!("\x1b[93m", $fmt, "\x1b[0m", "\n"), $($arg)*) );
    ($fmt:expr) => ( print!(concat!(yellowify!($fmt), "\n")) );
    ($fmt:expr, $($arg:tt)*) => ( print!(concat!(yellowify!($fmt), "\n"), $($arg)*) );
}

macro_rules! yellowify {
    ($s:expr) => (concat!("\x1b[93m", $s, "\x1b[0m"));
}

pub use self::cycle_loop::CycleLoop;
pub use self::cycle_result::{CyRes, CyStatus};
pub use self::cycle_control::CyCtl;
use std::str::{ FromStr };

mod cycle_loop;
mod cycle_result;
mod cycle_control;
pub mod output_czar;

pub fn parse_iters(in_s: &str) -> Result<u32, <u32 as FromStr>::Err> {
    in_s.trim().replace("k","000").parse()
    //in_s.trim().replace("m","000000").parse().ok()
}
