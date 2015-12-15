use time::{self, Timespec, Duration};


pub struct WinStats {
	pub frame_count: usize,
	pub start_time: Timespec,
	prev_event: Timespec,
	cur_fps: f32,
}

#[allow(dead_code)]
impl WinStats {
	pub fn new() -> WinStats {
		WinStats {
			frame_count: 0usize,
			start_time: time::get_time(),
			prev_event: time::get_time(),
			cur_fps: 0.0,
		}
	}

	pub fn fps(&self) -> f32 {
		// (self.event_count as f32 / (time::get_time() - self.start_time)
		// 	.num_milliseconds() as f32) * 1000.0
		self.cur_fps
	}

	pub fn elapsed_secs(&self) -> f32 {
		(time::get_time() - self.start_time).num_seconds() as f32
	}

	/// Returns microseconds elapsed since the window was created (mu = μ).
	pub fn elapsed_mus(&self) -> f64 {
		(time::get_time() - self.start_time).num_microseconds().unwrap() as f64
	}

	/// Returns milliseconds elapsed since the window was created.
	pub fn elapsed_ms(&self) -> f64 {
		(time::get_time() - self.start_time).num_milliseconds() as f64
	}

	/// Increment the frame counter by one and calculate fps for previous frame.
	pub fn incr(&mut self) {
		let now = time::get_time();

		let prev_frame_dur = now - self.prev_event;
		self.cur_fps = Duration::seconds(1).num_microseconds().unwrap() as f32
			/ prev_frame_dur.num_microseconds().unwrap() as f32;

		self.frame_count += 1;
		self.prev_event = now;
	}
}

