#![allow(dead_code)]

use glium_text::{TextSystem, FontTexture, TextDisplay};
use super::{UiVertex, TextAlign, MainWindow};
// use window::MainWindow;

pub const TEXT_SCALE: f32 = 0.54;
pub const DEFAULT_SCALE: f32 = 0.06;

struct TextProperties {
	string: String,
	base_scale: f32,	
	base_offset: (f32, f32),
	align: TextAlign,
	cur_scale: (f32, f32),
	cur_position: (f32, f32),
	cur_raw_width: f32,	
}

impl TextProperties {
	fn new(string: String) -> TextProperties {
		TextProperties {
			string: string,
			base_scale: TEXT_SCALE,
			cur_raw_width: 0.0,
			cur_scale: (0.0, 0.0), 
			cur_position: (0.0, 0.0),
			base_offset: (0.0, 0.0),
			align: TextAlign::Center,
		}
	}
}

// [FIXME]: TODO: 
// - Revamp 'new()' into builder style functions.
// - Clean up and consolidate stored positions, scales, etc.
pub struct UiElement {
	click_action: Option<Box<FnMut(&mut MainWindow)>>,
	vertices_pane_idz: Option<usize>,
	vertices_raw: Vec<UiVertex>,
	indices_raw: Vec<u16>,
	mouse_radii: (f32, f32),
	anchor_pos: [f32; 3],
	anchor_ofs: [f32; 3], 
	element_scale: (f32, f32),
	text: TextProperties,
	text_string: String,
	sub_elements: Vec<UiElement>,
	cur_scale: [f32; 3],
	cur_position: [f32; 3],
}

impl<'a> UiElement {
	pub fn new(anchor_pos: [f32; 3], anchor_ofs: [f32; 3], element_scale: (f32, f32), vertices_raw: Vec<UiVertex>, 
				 indices_raw: Vec<u16>, text: String, mouse_radii: (f32, f32),
			) -> UiElement
	{
		verify_position(anchor_pos);

		UiElement { 
			click_action: None,
			vertices_pane_idz: None,
			vertices_raw: vertices_raw, 
			indices_raw: indices_raw,
			mouse_radii: mouse_radii,
			anchor_pos: anchor_pos,
			anchor_ofs: anchor_ofs,
			element_scale: element_scale,
			text: TextProperties::new(text.clone()),
			text_string: text,
			sub_elements: Vec::with_capacity(0),
			cur_scale: [0.0, 0.0, 0.0],
			cur_position: [0.0, 0.0, 0.0],	
		}
	}

	pub fn click_action(mut self, click_action: Box<FnMut(&mut MainWindow)>) -> UiElement {
		self.click_action = Some(click_action);
		self
	}

	pub fn sub(mut self, sub_element: UiElement) -> UiElement {
		self.sub_elements.reserve_exact(1);
		self.sub_elements.push(sub_element);
		self
	}

	pub fn text_offset(mut self, base_offset: (f32, f32)) -> UiElement {
		self.text.base_offset = base_offset;
		self
	}

	pub fn vertices_raw(&self) -> &[UiVertex] {
		&self.vertices_raw[..]
	}

	pub fn indices_raw(&self) -> &[u16] {
		&self.indices_raw[..]
	}

	pub fn vertices(&mut self, window_dims: (u32, u32), ui_scale: f32, vertices_pane_idz: usize) -> Vec<UiVertex> {
		self.vertices_pane_idz = Some(vertices_pane_idz);
		let ar = window_dims.0 as f32 / window_dims.1 as f32;	

		self.cur_scale = [self.element_scale.0 * ui_scale / ar, self.element_scale.1 * ui_scale, ui_scale];
		
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
			((-self.cur_scale[0] * self.text.cur_raw_width / 2.0) * self.text.base_scale) 
				+ self.cur_position[0]
				+ (self.text.base_offset.0 * self.cur_scale[0]), 
			((-self.cur_scale[1] / 2.0) * self.text.base_scale) 
				+ self.cur_position[1]
				+ (self.text.base_offset.1 * self.cur_scale[1]), 
		);

		// [FIXME]: TODO: Convert all of this to a collect():
		let mut vertices = Vec::with_capacity(self.vertices_raw.len());

		// print!("\nVertices positions:  ");

		for &vertex in self.vertices_raw.iter() {
			let new_vertex = vertex.transform(&self.cur_scale, &self.cur_position);
			// print!("{:?}", vertex.position());
			vertices.push(new_vertex);
		}

		vertices
	}

	/// Returns the list of indices with 'shift_by' added to each one.
	pub fn indices(&self, shift_by: u16) -> Vec<u16> {
		self.indices_raw.iter().map(|&ind| ind + shift_by).collect()
	}

	pub fn set_text_width(&mut self, ts: &TextSystem, ft: &FontTexture) {
		let text_display = TextDisplay::new(ts, ft, &self.text.string);
		self.text.cur_raw_width = text_display.get_width();
	}

	pub fn position(&self) -> [f32; 3] {
		self.cur_position
	}

	pub fn scale(&self) -> [f32; 3] {
		self.cur_scale
	}

	pub fn text(&self) -> &str {
		&self.text.string
	}

	pub fn set_color(&mut self, color: [f32; 3]) {
		for vertex in self.vertices_raw.iter_mut() {
			vertex.set_color(color);
		}
	}

	pub fn text_matrix(&self) -> [[f32; 4]; 4] {
		[	
			[self.text.cur_scale.0, 0.0, 0.0, 0.0,],
			[0.0, self.text.cur_scale.1, 0.0, 0.0,],
			[0.0, 0.0, 1.0, 0.0,],
			[self.text.cur_position.0, self.text.cur_position.1, 0.0, 1.0f32,], 
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

	pub fn click(&mut self, mw: &mut MainWindow) {
		if let Some(ref mut ca) = self.click_action {
			ca(mw);
		}
	}

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
