#![allow(dead_code)]

use glium_text::{TextSystem, FontTexture, TextDisplay};
use window::{self, UiVertex, TextAlign, MainWindow};

pub const TEXT_SCALE: f32 = 0.54;
pub const DEFAULT_SCALE: f32 = 0.06;

/*
	Notes:
	- 'raw' is intended to mean something based on a position which is unscaled by the screen and generally has a height of roughly 1.0f32.
	- 'cur' is a pre-calculated value containing information about the current screen state (such as its size) and is used as a cached value.

	- 'idz' is, as always, the index of item[0] within a larger set (think memory location).

*/


// [FIXME]: TODO: 
// - Revamp 'new()' into builder style functions.
// - Clean up and consolidate stored positions, scales, etc.
pub struct UiElement {
	text: TextProperties,
	sub_elements: Vec<UiElement>,
	click_action: Option<Box<FnMut(&mut MainWindow)>>,
	vertices_raw: Vec<UiVertex>,
	indices_raw: Vec<u16>,
	mouse_radii: (f32, f32),
	anchor_pos: [f32; 3],
	anchor_ofs: [f32; 3], 
	base_scale: (f32, f32),
	cur_scale: [f32; 3],
	cur_position: [f32; 3],
}

impl<'a> UiElement {
	pub fn new(anchor_pos: [f32; 3], anchor_ofs: [f32; 3], vertices_raw: Vec<UiVertex>, 
				 indices_raw: Vec<u16>, mouse_radii: (f32, f32),
			) -> UiElement
	{
		verify_position(anchor_pos);

		UiElement { 
			text: TextProperties::new("".to_string()),
			sub_elements: Vec::with_capacity(0),
			click_action: None,
			vertices_raw: vertices_raw, 
			indices_raw: indices_raw,
			mouse_radii: mouse_radii,
			anchor_pos: anchor_pos,
			anchor_ofs: anchor_ofs,
			base_scale: (DEFAULT_SCALE, DEFAULT_SCALE),			
			cur_scale: [0.0, 0.0, 0.0],
			cur_position: [0.0, 0.0, 0.0],	
		}
	}

	pub fn click_action(mut self, click_action: Box<FnMut(&mut MainWindow)>) -> UiElement {
		self.click_action = Some(click_action);
		self
	}

	pub fn sub(mut self, mut sub_element: UiElement) -> UiElement {
		sub_element.anchor_pos[2] += window::SUBDEPTH;
		self.sub_elements.reserve_exact(1);
		self.sub_elements.push(sub_element);
		self
	}

	pub fn text(mut self, text_string: String) -> UiElement {
		self.text.string = text_string;
		self
	}

	pub fn text_offset(mut self, element_offset: (f32, f32)) -> UiElement {
		self.text.element_offset = element_offset;
		self
	}

	pub fn vertices_raw(&self) -> &[UiVertex] {
		&self.vertices_raw[..]
	}

	pub fn indices_raw(&self) -> &[u16] {
		&self.indices_raw[..]
	}

	pub fn vertices(&mut self, window_dims: (u32, u32), ui_scale: f32) -> Vec<UiVertex> {
		let ar = window_dims.0 as f32 / window_dims.1 as f32;	

		self.cur_scale = [self.base_scale.0 * ui_scale / ar, self.base_scale.1 * ui_scale, ui_scale];
		
		self.cur_position = [
			self.anchor_pos[0] + ((self.anchor_ofs[0] / ar) * ui_scale),
			self.anchor_pos[1] + (self.anchor_ofs[1] * ui_scale),
			(self.anchor_pos[2] + self.anchor_ofs[2]) * ui_scale,
		];

		self.text.cur_scale = (
			self.cur_scale[0] * self.text.base_scale, 
			self.cur_scale[1] * self.text.base_scale
		);

		self.text.cur_position = (
			((-self.cur_scale[0] * self.text.raw_width / 2.0) * self.text.base_scale) 
				+ self.cur_position[0]
				+ (self.text.element_offset.0 * self.cur_scale[0]), 
			((-self.cur_scale[1] / 2.0) * self.text.base_scale) 
				+ self.cur_position[1]
				+ (self.text.element_offset.1 * self.cur_scale[1]), 
		);

		let mut vertices: Vec<UiVertex> = self.vertices_raw.iter().map(
			|&vrt| vrt.transform(&self.cur_scale, &self.cur_position)).collect();

		for sub_ele in self.sub_elements.iter_mut() {
			vertices.extend_from_slice(&sub_ele.vertices(window_dims.clone(), ui_scale));
		}

		vertices
	}

	/// Returns the list of indices with 'shift_by' added to each one.
	pub fn indices(&self, shift_by: u16) -> Vec<u16> {
		let mut indices: Vec<u16> = self.indices_raw.iter().map(|&ind| ind + shift_by).collect();

		let mut sub_shift_by = shift_by + (self.vertices_raw.len() as u16);

		for sub_ele in self.sub_elements.iter() {
			indices.extend_from_slice(&sub_ele.indices(sub_shift_by));
			sub_shift_by += sub_ele.vertices_raw.len() as u16;
		}

		indices
	}

	pub fn set_text_width(&mut self, ts: &TextSystem, ft: &FontTexture) {
		// let text_display = TextDisplay::new(ts, ft, &self.text.string);
		// self.text.raw_width = text_display.get_width();
		self.text.set_raw_width(ts, ft);
	}

	pub fn position(&self) -> [f32; 3] {
		self.cur_position
	}

	pub fn scale(&self) -> [f32; 3] {
		self.cur_scale
	}

	pub fn get_text(&self) -> &str {
		&self.text.string
	}

	pub fn set_color(&mut self, color: [f32; 3]) {
		for vertex in self.vertices_raw.iter_mut() {
			vertex.set_color(color);
		}
	}

	pub fn text_matrix(&self) -> [[f32; 4]; 4] {
		self.text.matrix()
	}

	pub fn has_mouse_focus(&self, mouse_pos: (f32, f32)) -> bool {
		mouse_pos.0 >= self.left_edge() && mouse_pos.0 <= self.right_edge()
			&& mouse_pos.1 <= self.top_edge() && mouse_pos.1 >= self.bottom_edge()
	}

	pub fn click(&mut self, mw: &mut MainWindow) {
		if let Some(ref mut ca) = self.click_action {
			ca(mw);
		}
	}

	///////// CACHE THIS STUFF //////////
	fn left_edge(&self) -> f32 {
		self.cur_position[0] - (self.mouse_radii.0 * self.cur_scale[0])
	}

	fn right_edge(&self) -> f32 {
		self.cur_position[0] + (self.mouse_radii.0 * self.cur_scale[0])
	}

	fn top_edge(&self) -> f32 {
		self.cur_position[1] + (self.mouse_radii.1 * self.cur_scale[1])
	}

	fn bottom_edge(&self) -> f32 {
		self.cur_position[1] - (self.mouse_radii.1 * self.cur_scale[1])
	}
	//////////////////////////////////////

}


pub struct TextProperties {
	string: String,
	base_scale: f32,	
	element_offset: (f32, f32),
	align: TextAlign,
	raw_width: f32,	
	cur_scale: (f32, f32),
	cur_position: (f32, f32),
}

impl TextProperties {
	pub fn new(string: String) -> TextProperties {
		TextProperties {
			string: string,
			base_scale: TEXT_SCALE,
			element_offset: (0.0, 0.0),
			align: TextAlign::Center,
			raw_width: 0.0,
			cur_scale: (0.0, 0.0), 
			cur_position: (0.0, 0.0),
		}
	}

	pub fn matrix(&self) -> [[f32; 4]; 4] {
		[	[self.cur_scale.0, 0.0, 0.0, 0.0,],
			[0.0, self.cur_scale.1, 0.0, 0.0,],
			[0.0, 0.0, 1.0, 0.0,],
			[self.cur_position.0, self.cur_position.1, 0.0, 1.0f32,], 	]
	}

	pub fn set_raw_width(&mut self, ts: &TextSystem, ft: &FontTexture) {
		let text_display = TextDisplay::new(ts, ft, &self.string);
		self.raw_width = text_display.get_width();
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
