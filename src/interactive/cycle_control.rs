use std::sync::{Arc, Mutex};

#[allow(dead_code)]
pub enum CyCtl {
	None,
	Iterate(u32),
	Sample(Arc<Mutex<Vec<u8>>>),
	RequestCurrentAreaName,
	// ViewAllSlices(bool),
	// ViewEnvoyDebug(bool),
	Stop,
	Exit,
}
