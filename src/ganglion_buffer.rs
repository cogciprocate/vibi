use std::iter;
use std::sync::{Arc, Mutex};
use rand;
use rand::distributions::{IndependentSample, Range as RandRange};
use glium::backend::glutin_backend::{GlutinFacade};
use glium::VertexBuffer;


pub struct GanglionBuffer {	
	raw_states: Arc<Mutex<Vec<u8>>>,
	state_vertices: Vec<StateVertex>,
	v_buf: VertexBuffer<StateVertex>,
}

impl GanglionBuffer {
	pub fn new(grid_count: usize, display: &GlutinFacade) -> GanglionBuffer {
		let raw_states = iter::repeat(0u8).cycle().take(grid_count).collect();
		let state_vertices: Vec<StateVertex> = iter::repeat(StateVertex { state: 0.0 })
			.cycle().take(grid_count).collect();
		let v_buf = VertexBuffer::dynamic(display, &state_vertices).unwrap();

		GanglionBuffer {			
			raw_states: Arc::new(Mutex::new(raw_states)),
			state_vertices: state_vertices,
			v_buf: v_buf,
		}
	}

	pub fn refresh_v_buf(&mut self) {
		let raw_states_ptr = self.raw_states.clone();
		let raw_states = raw_states_ptr.lock().unwrap();
		debug_assert!(raw_states.len() == self.state_vertices.len());

		for (&rs, ref mut sv) in raw_states.iter().zip(self.state_vertices.iter_mut()) {
			sv.state = (rs as f32) / 255.0;
		}

		self.v_buf.write(&self.state_vertices);
	}

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
