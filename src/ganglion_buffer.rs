use std::iter;
use rand::distributions::{IndependentSample, Range as RandRange};
use rand::{self, XorShiftRng};
use glium::backend::glutin_backend::{GlutinFacade};
use glium::VertexBuffer;


pub struct GanglionBuffer {
	buf: VertexBuffer<StateVertex>,
	rng: XorShiftRng,
}

impl GanglionBuffer {
	pub fn new(grid_count: usize, display: &GlutinFacade) -> GanglionBuffer {

		let mut rng = rand::weak_rng();
		let range = RandRange::new(0.0f32, 1.0);		
		// let gc_half = grid_count / 2;

		// Make a bullshit 'plank':
		let p_vec: Vec<StateVertex> = iter::repeat(0).cycle().take(grid_count)
				.map(|_| StateVertex { state: range.ind_sample(&mut rng) })
			// .chain(iter::repeat(0.5f32).cycle().take(grid_count - gc_half).map(|v| StateVertex { state : v }))
			.collect();

		let buf = VertexBuffer::dynamic(display, &p_vec).unwrap();

		GanglionBuffer {
			buf: buf,
			rng: rng,
		}
	}

	pub fn buf(&self) -> &VertexBuffer<StateVertex> {
		&self.buf
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
