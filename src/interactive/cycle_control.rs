use std::ops::Range;
use std::sync::{Arc, Mutex};

#[allow(dead_code)]
pub enum CyCtl {
	None,
	Iterate(u32),
	Sample(Range<usize>, Arc<Mutex<Vec<u8>>>),
	RequestCurrentAreaInfo,
	// ViewAllSlices(bool),
	// ViewEnvoyDebug(bool),
	Stop,
	Exit,
}
