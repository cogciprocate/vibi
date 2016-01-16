use time::Duration;

pub enum CyRes {

}

#[derive(Clone)]
pub struct CyStatus {
	pub dims: (u32, u32),
	pub cur_cycle: u32,
	pub ttl_cycles: u32,
	pub cur_elapsed: Duration,
	pub ttl_elapsed: Duration,
}

#[allow(dead_code)]
impl CyStatus {
	pub fn new(dims: (u32, u32)) -> CyStatus {
		CyStatus {
			dims: dims,
			cur_cycle: 0,
			ttl_cycles: 0,
			cur_elapsed: Duration::seconds(0),
			ttl_elapsed: Duration::seconds(0),
		}
	}

	pub fn ttl_cps(&self) -> f32 {
		// if self.ttl_elapsed.num_milliseconds() > 0 {
		// 	(self.ttl_cycles as f32 / self.ttl_elapsed.num_milliseconds() as f32) * 1000.0
		// } else {
		// 	0.0
		// }
		cps(self.ttl_cycles, self.ttl_elapsed)
	}

	pub fn cur_cps(&self) -> f32 {
		// if self.cur_elapsed.num_milliseconds() > 0 {
		// 	(self.cur_cycle as f32 / self.cur_elapsed.num_milliseconds() as f32) * 1000.0
		// } else {
		// 	0.0
		// }
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
