pub use self::cycle::{CycleLoop, CyCtl, CyRes, CyStatus};
// pub use self::cycle_result::{CyRes, CyStatus};
// pub use self::cycle_control::CyCtl;
use std::str::{ FromStr };

mod cycle;
// mod cycle_result;
// mod cycle_control;
// pub mod output_czar;

pub fn parse_iters(in_s: &str) -> Result<u32, <u32 as FromStr>::Err> {
    in_s.trim().replace("k","000").replace("m","000000").parse()
    // in_s.trim().replace("m","000000").parse().ok()
}
