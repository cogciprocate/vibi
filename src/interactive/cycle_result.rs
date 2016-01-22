use std::ops::Range;
use time::Duration;
use bismit::map::GanglionMap;


/// Cycle result. 
///
/// Information about the cycling of the things and the stuff (and some of the non-stuff too... but not that much of it really... well... a fair amount... but not like a ton).
#[derive(Clone)]
#[allow(dead_code)]
pub enum CyRes {
	None,
	Status(CyStatus),
	CurrentAreaInfo(String, Range<u8>, GanglionMap),
	// OtherShit(GanglionMap),
}

#[derive(Clone)]
pub struct CyStatus {
	// pub dims: (u32, u32),
	pub cur_cycle: u32,
	pub ttl_cycles: u32,
	pub cur_elapsed: Duration,
	pub ttl_elapsed: Duration,
}

#[allow(dead_code)]
impl CyStatus {
	pub fn new(/*dims: (u32, u32)*/) -> CyStatus {
		CyStatus {
			// dims: dims,
			cur_cycle: 0,
			ttl_cycles: 0,
			cur_elapsed: Duration::seconds(0),
			ttl_elapsed: Duration::seconds(0),
		}
	}

	pub fn ttl_cps(&self) -> f32 {
		cps(self.ttl_cycles, self.ttl_elapsed)
	}

	pub fn cur_cps(&self) -> f32 {
		cps(self.cur_cycle, self.cur_elapsed)
	}
}

fn cps(cycle: u32, elapsed: Duration) -> f32 {
	if elapsed.num_milliseconds() > 0 {
		(cycle as f32 / elapsed.num_milliseconds() as f32) * 1000.0
	} else {
		0.0
	}
}
