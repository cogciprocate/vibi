#![allow(dead_code)]
use glium_text::{TextSystem, FontTexture, TextDisplay};
use super::{UiVertex};

pub struct UiElement {
	vertices: Vec<UiVertex>,
	indices: Vec<u16>,
	radii: (f32, f32),
	anchor_pos: [f32; 3],
	offset: (f32, f32), 
	scale: (f32, f32),
	scale_vec: [f32; 3],
	position_vec: [f32; 3],	
	text: String,
	text_scale: f32,
	text_width: f32,
	txt_scl: [f32; 3],
	txt_pos: [f32; 3],
}

impl UiElement {
	pub fn new(anchor_pos: [f32; 3], offset: (f32, f32), scale: (f32, f32), vertices: Vec<UiVertex>, 
				 indices: Vec<u16>, text: String, radii: (f32, f32),
			) -> UiElement
	{
		verify_position(anchor_pos);

		UiElement { 
			vertices: vertices, 
			indices: indices,
			radii: radii,
			anchor_pos: anchor_pos,
			offset: offset,
			scale: scale,
			scale_vec: [0.0, 0.0, 0.0],
			position_vec: [0.0, 0.0, 0.0],
			text: text,
			text_scale: 0.45,
			text_width: 1.0,
			txt_scl: [0.0, 0.0, 0.0], 
			txt_pos: [0.0, 0.0, 0.0],			
		}
	}

	pub fn hex_button(anchor_pos: [f32; 3], offset: (f32, f32), scale: f32, extra_width: f32,
			text: String, color: [f32; 3]) -> UiElement
	{
		// NOTE: width(x): 1.15470053838 (2/sqrt(3)), height(y): 1.0
		let ew = extra_width;
		let a = 0.5;
		let s = 0.57735026919; // 1/sqrt(3)
		let hs = s * 0.5;

		let vertices = vec![
			UiVertex::new([ 0.0, 		 0.0, 	 0.0], color, [0.0, 0.0, -1.0]),
			UiVertex::new([-(hs + ew),	 a,  	 0.0], color, [0.0, 0.0, -1.0]),
			UiVertex::new([ hs + ew, 	 a,  	 0.0], color, [0.0, 0.0, -1.0]),
			UiVertex::new([ s + ew, 	 0.0,  	 0.0], color, [0.0, 0.0, -1.0]),
			UiVertex::new([ hs + ew, 	-a, 	 0.0], color, [0.0, 0.0, -1.0]),
			UiVertex::new([-(hs + ew), 	-a,  	 0.0], color, [0.0, 0.0, -1.0]),
			UiVertex::new([-(s + ew),  	 0.0,  	 0.0], color, [0.0, 0.0, -1.0]),
		];

		let indices = vec![
			0, 1, 2,
			2, 3, 0,
			0, 3, 4,
			4, 5, 0,
			0, 5, 6,
			6, 1, 0u16,
		];

		// Distance from center to right edge (splitting the difference):
		let x_radius = ew + (s * 0.75);
		let y_radius = a;

		UiElement::new(anchor_pos, offset, (scale, scale), vertices, indices, text, (x_radius, y_radius))
	}

	pub fn vertices_raw(&self) -> &[UiVertex] {
		&self.vertices[..]
	}

	pub fn indices_raw(&self) -> &[u16] {
		&self.indices[..]
	}

	pub fn vertices(&mut self, window_dims: (u32, u32), scale: f32) -> Vec<UiVertex> {
		let ar = window_dims.0 as f32 / window_dims.1 as f32;	

		self.scale_vec = [self.scale.0 * scale / ar, self.scale.1 * scale, 1.0];
		
		self.position_vec = [
			self.anchor_pos[0] + (self.offset.0 / ar),
			self.anchor_pos[1] + self.offset.1,
			0.0,
		];

		self.txt_scl = [
			self.scale_vec[0] * self.text_scale, 
			self.scale_vec[1] * self.text_scale, 
			0.0
		];

		self.txt_pos = [
			((-self.scale_vec[0] * self.text_width / 2.0) * self.text_scale) + self.position_vec[0], 
			((-self.scale_vec[1] / 2.0) * self.text_scale) + self.position_vec[1],
			0.0
		];

		// [FIXME]: TODO: Convert all of this to a collect():
		let mut vertices = Vec::with_capacity(self.vertices.len());

		for &vertex in self.vertices.iter() {
			vertices.push(vertex.transform(&self.scale_vec, &self.position_vec));
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

	pub fn set_text_width(&mut self, ts: &TextSystem, ft: &FontTexture) {
		let text_display = TextDisplay::new(ts, ft, &self.text);
		self.text_width = text_display.get_width();
	}

	pub fn position(&self) -> [f32; 3] {
		self.position_vec
	}

	pub fn scale(&self) -> [f32; 3] {
		self.scale_vec
	}

	pub fn text(&self) -> &str {
		&self.text
	}

	pub fn set_color(&mut self, color: [f32; 3]) {
		for vertex in self.vertices.iter_mut() {
			vertex.set_color(color);
		}
	}

	pub fn text_matrix(&self) -> [[f32; 4]; 4] {
		[	
			[self.txt_scl[0], 0.0, 0.0, 0.0,],
			[0.0, self.txt_scl[1], 0.0, 0.0,],
			[0.0, 0.0, 1.0, 0.0,],
			[self.txt_pos[0], self.txt_pos[1], 0.0, 1.0f32,], 
		]
	}

	pub fn has_mouse_focus(&self, mouse_pos: (f32, f32)) -> bool {
		// print!("    ");
		// print!("Top: {:.2}, ", self.top_edge());
		// print!("Bottom: {:.2}, ", self.bottom_edge());
		// print!("Left: {:.2}, ", self.left_edge());
		// print!("Right: {:.2}, ", self.right_edge());
		// print!("{{ Mouse ({:.2}, {:.2}) }}", mouse_pos.0, mouse_pos.1);
		// print!("\n");

		mouse_pos.0 >= self.left_edge() && mouse_pos.0 <= self.right_edge()
			&& mouse_pos.1 <= self.top_edge() && mouse_pos.1 >= self.bottom_edge()
	}

	fn left_edge(&self) -> f32 {
		self.position_vec[0] - (self.radii.0 * self.scale_vec[0])
	}

	fn right_edge(&self) -> f32 {
		self.position_vec[0] + (self.radii.0 * self.scale_vec[0])
	}

	fn top_edge(&self) -> f32 {
		self.position_vec[1] + (self.radii.1 * self.scale_vec[1])
	}

	fn bottom_edge(&self) -> f32 {
		self.position_vec[1] - (self.radii.1 * self.scale_vec[1])
	}
}

// Ensure position is within -1.0 and 1.0 for x and y dims.
fn verify_position(position: [f32; 3]) {
	assert!((position[0] <= 1.0 && position[0] >= -1.0) 
			|| (position[1] <= 1.0 && position[1] >= -1.0), 
		format!("UiElement::new(): Position out of range: [x: {}, y: {}, z:{}]. \
			'x' and 'y' must both be between -1.0 and 1.0.", 
			position[0], position[1], position[2])
	);
}
