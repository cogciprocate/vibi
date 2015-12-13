#![allow(dead_code)]
use super::{ UiVertex };

pub struct UiElement {
	vertices: Vec<UiVertex>,
	indices: Vec<u16>,
	anchor_pos: [f32; 3],
	offset: (i32, i32), 
	size: (u32, u32),
}

impl UiElement {
	pub fn new(anchor_pos: [f32; 3], offset: (i32, i32), size: (u32, u32)) -> UiElement {
		verify_pos(anchor_pos);
		// NOTE: width(x): 1.15470053838 (2/sqrt(3)), height(y): 1.0
		let scale =  0.1;
		let a = 0.5 * scale;
		let s = 0.57735026919 * scale; // 1/sqrt(3)
		let hs = s / 2.0;

		let vertices = vec![
			UiVertex::new([ 0.0, 	 0.0, 	 0.0], [0.4, 0.4, 0.4,], [0.0, 0.0, -1.0]),
			UiVertex::new([-hs, 	 a,  	 0.0], [0.7, 0.7, 0.2,], [0.0, 0.0, -1.0]),
			UiVertex::new([ hs, 	 a,  	 0.0], [0.2, 0.7, 0.7,], [0.0, 0.0, -1.0]),
			UiVertex::new([ s, 	 	 0.0,  	 0.0], [0.7, 0.2, 0.7,], [0.0, 0.0, -1.0]),
			UiVertex::new([ hs, 	-a, 	 0.0], [0.7, 0.7, 0.2,], [0.0, 0.0, -1.0]),
			UiVertex::new([-hs, 	-a,  	 0.0], [0.2, 0.7, 0.7,], [0.0, 0.0, -1.0]),
			UiVertex::new([-s, 	 	 0.0,  	 0.0], [0.7, 0.2, 0.7,], [0.0, 0.0, -1.0]),
		];

		let indices = vec![
			0, 1, 2,
			2, 3, 0,
			0, 3, 4,
			4, 5, 0,
			0, 5, 6,
			6, 1, 0u16,
		];

		UiElement { 
			vertices: vertices, 
			indices: indices,
			anchor_pos: anchor_pos,
			offset: offset,
			size: size,
		}
	}

	pub fn vertices_raw(&self) -> &[UiVertex] {
		&self.vertices[..]
	}

	pub fn indices_raw(&self) -> &[u16] {
		&self.indices[..]
	}

	pub fn vertices(&self, window_dims: (u32, u32), scale: f32) -> Vec<UiVertex> {
		// -- Scale --
		// let scl: f32 = (scale) as f32;
		// let scale_vec: [f32; 2] = 

		// -- Position --
		let window_center_xy: (i32, i32) = 
			(window_dims.0 as i32 / 2, window_dims.1 as i32 / 2);

		let anchor_xy: (i32, i32) = (
			(self.anchor_pos[0] * window_center_xy.0 as f32) as i32,
			(self.anchor_pos[1] * window_center_xy.1 as f32) as i32,
		);

		println!("### anchor_xy: {:?}", anchor_xy);

		let element_center_xy: (i32, i32) = (
			anchor_xy.0 + self.offset.0,
			anchor_xy.1 + self.offset.1,
		);

		// [FIXME]: Handle aspect ratio ourselves?... not until dims are handled.
		// Aspect ratio handled by 'Ui' so give x_coord in terms of y_dim.
		let shift_vec: [f32; 3] = [
			element_center_xy.0 as f32 / window_center_xy.1 as f32,
			element_center_xy.1 as f32 / window_center_xy.1 as f32,
			0.0,
		];
		

		// [FIXME]: TODO: Convert all of this to a collect():
		let mut vertices = Vec::with_capacity(self.vertices.len());

		for &vertex in self.vertices.iter() {
			vertices.push(vertex.shift(&shift_vec));
		}

		vertices
	}

	/// Returns the list of indices with 'shift_by' added to each one.
	pub fn indices(&self, shift_by: u16) -> Vec<u16> {
		let mut indices_shifted = Vec::with_capacity(self.indices.len());

		for &index in self.indices.iter() {
			indices_shifted.push(index + shift_by);
		}

		indices_shifted
	}
}

fn verify_pos(position: [f32; 3]) {
	assert!((position[0] <= 1.0 && position[0] >= -1.0) 
			|| (position[1] <= 1.0 && position[1] >= -1.0), 
		format!("UiElement::new(): Position out of range: [x: {}, y: {}, z:{}]. \
			'x' and 'y' must both be between -1.0 and 1.0.", 
			position[0], position[1], position[2])
	);
}


// fn vbo(display: &GlutinFacade) -> VertexBuffer<UiVertex> {
// 	// NOTE: width(x): 1.15470053838 (2/sqrt(3)), height(y): 1.0
// 	let a = 0.5;
// 	let s = 0.57735026919; // 1/sqrt(3)
// 	let hs = s / 2.0f32;

// 	glium::vertex::VertexBuffer::new(display, &[
// 			UiVertex::new([ 0.0, 	 0.0, 	 0.0], [0.4, 0.4, 0.4,], [0.0, 0.0, -1.0]),
// 			UiVertex::new([-hs, 	 a,  	 0.0], [0.7, 0.7, 0.2,], [0.0, 0.0, -1.0]),
// 			UiVertex::new([ hs, 	 a,  	 0.0], [0.2, 0.7, 0.7,], [0.0, 0.0, -1.0]),
// 			UiVertex::new([ s, 	 0.0,  	 0.0], [0.7, 0.2, 0.7,], [0.0, 0.0, -1.0]),
// 			UiVertex::new([ hs, 	-a, 	 0.0], [0.7, 0.7, 0.2,], [0.0, 0.0, -1.0]),
// 			UiVertex::new([-hs, 	-a,  	 0.0], [0.2, 0.7, 0.7,], [0.0, 0.0, -1.0]),
// 			UiVertex::new([-s, 	 0.0,  	 0.0], [0.7, 0.2, 0.7,], [0.0, 0.0, -1.0]),
// 		]).unwrap()
// }


// fn ibo(display: &GlutinFacade) -> IndexBuffer<u16> {
// 	glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &[
// 			0, 1, 2,
// 			2, 3, 0,
// 			0, 3, 4,
// 			4, 5, 0,
// 			0, 5, 6,
// 			6, 1, 0u16,
// 		]).unwrap()
// }

