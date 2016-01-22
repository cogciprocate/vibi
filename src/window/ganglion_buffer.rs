use std::iter;
use std::sync::{Arc, Mutex};
use std::ops::Range;
use rand;
use rand::distributions::{IndependentSample, Range as RandRange};
use glium::backend::glutin_backend::{GlutinFacade};
use glium::VertexBuffer;
use bismit::map::GanglionMap;

/// Handles raw state data from a cortical ganglion and feeds it to a [vertex] buffer for rendering.
// TODO: Rename these buffers to something more clear.
pub struct GanglionBuffer {
	raw_states: Arc<Mutex<Vec<u8>>>,
	state_vertices: Vec<StateVertex>,
	v_buf: VertexBuffer<StateVertex>,
	total_slc_range: Range<u8>,
	default_slc_range: Range<u8>,
	current_slc_range: Range<u8>,
	gang_map: GanglionMap,
}

impl GanglionBuffer {
	pub fn new(default_slc_range: Range<u8>, gang_map: GanglionMap, display: &GlutinFacade) 
			-> GanglionBuffer 
	{
		let grid_count = gang_map.axn_count(default_slc_range.clone());

		// println!("\n###### GANGLIONBUFFER: d_slc_range: {:?}, grid_count: {}, gang_map: {:?}\n", 
		// 	default_slc_range, grid_count, gang_map);

		let raw_states = iter::repeat(0u8).cycle().take(grid_count).collect();
		let state_vertices: Vec<StateVertex> = iter::repeat(StateVertex { state: 0.0 })
			.cycle().take(grid_count).collect();
		let v_buf = VertexBuffer::dynamic(display, &state_vertices).unwrap();

		GanglionBuffer {			
			raw_states: Arc::new(Mutex::new(raw_states)),
			state_vertices: state_vertices,
			v_buf: v_buf,
			total_slc_range: gang_map.slc_range(),
			default_slc_range: default_slc_range.clone(),
			current_slc_range: default_slc_range.clone(),
			gang_map: gang_map,
		}
	}

	/// Refreshes the per-instance data within our vertex buffer.
	///
	///  *If* a lock on `raw_states` can be obtained (if it's not currently being written to by another thread): converts the u8s in `raw_states` to floats, store them in `state_vertices`, then writes the contents of `state_vertices` to `v_buf`.
	///
	/// This is an opportunistic refresh, it will sometimes do nothing at all.
	pub fn refresh_v_buf(&mut self) {
		let raw_states_ptr = self.raw_states.clone();
		// Change to .lock() for smoother refreshes at the cost of slower cycling:
		let raw_states_res = raw_states_ptr.try_lock();
		
		if let Ok(ref raw_states) = raw_states_res {
			debug_assert!(raw_states.len() == self.state_vertices.len());

			for (&rs, ref mut sv) in raw_states.iter().zip(self.state_vertices.iter_mut()) {
				sv.state = (rs as f32) / 255.0;
			}

			self.v_buf.write(&self.state_vertices);
		}		
	}

	pub fn set_default_slc_range(&mut self, slc_range: Range<u8>) {
		self.default_slc_range = slc_range;
	}

	pub fn set_gang_map(&mut self, gang_map: GanglionMap) {
		self.gang_map = gang_map;
	}
 
 	// [FIXME]: DEPRICATE OR MOVE TO TESTS MODULE
	#[allow(dead_code)]
	pub fn fill_rand(&mut self) {
		let mut rng = rand::thread_rng();
		let range = RandRange::new(0u8, 255);

		let raw_states_ptr = self.raw_states.clone();
		let mut raw_states = raw_states_ptr.lock().unwrap();

		for rs in raw_states.iter_mut() {
			*rs = range.ind_sample(&mut rng);
		}
	}

	pub fn raw_states(&mut self) -> Arc<Mutex<Vec<u8>>> {
		self.raw_states.clone()
	}

	pub fn v_buf(&self) -> &VertexBuffer<StateVertex> {
		&self.v_buf
	}	
}


#[derive(Copy, Clone, Debug)]
pub struct StateVertex {
	state: f32,
}

// impl Vertex {
// 	fn new(position: [f32; 3], color: [f32; 3], normal: [f32; 3]) -> Vertex {
// 		Vertex { position: position, color: color, normal: normal }
// 	}
// }
implement_vertex!(StateVertex, state);
