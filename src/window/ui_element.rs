#![allow(dead_code)]

use glium::Surface;
use glium_text::{self, TextSystem, FontTexture, TextDisplay};
use glium::glutin::{ElementState, MouseButton, VirtualKeyCode};
use window::{self, UiVertex, MainWindow, TextProperties, HandlerOption, MouseInputHandler, 
	KeyboardInputHandler, MouseInputEventResult, KeyboardInputEventResult};

pub const ELEMENT_BASE_SCALE: f32 = 0.07;

/*********
	Notes:
	- 'raw' is intended to mean something based on a position which is unscaled by the screen and generally has a height of roughly 1.0f32.
	- 'cur' is a pre-calculated value containing information about the current screen state (such as its size) and is used as a cached value.

	- 'idz' is, as always, the index of item[0] within a larger set (think memory location).

*********/


// [FIXME]: TODO: 
// - Revamp 'new()' into builder style functions.
// - Clean up and consolidate stored positions, scales, etc.
pub struct UiElement {
	// kind: UiElementKind,
	text: TextProperties,
	sub_elements: Vec<UiElement>,	
	vertices_raw: Vec<UiVertex>,
	indices_raw: Vec<u16>,
	mouse_radii: (f32, f32),
	has_keybd_focus: bool,
	anchor_pos: [f32; 3],
	anchor_ofs: [f32; 3], 
	base_scale: (f32, f32),
	cur_scale: [f32; 3],
	cur_position: [f32; 3],
	mouse_input_handler: HandlerOption<MouseInputHandler>,
	keyboard_input_handler: HandlerOption<KeyboardInputHandler>,
}

impl<'a> UiElement {
	pub fn new(anchor_pos: [f32; 3], anchor_ofs: [f32; 3], vertices_raw: Vec<UiVertex>, 
				 indices_raw: Vec<u16>, mouse_radii: (f32, f32),
			) -> UiElement
	{
		verify_position(anchor_pos);

		UiElement { 
			text: TextProperties::new(""),
			sub_elements: Vec::with_capacity(0),
			vertices_raw: vertices_raw, 
			indices_raw: indices_raw,
			mouse_radii: mouse_radii,
			has_keybd_focus: false,
			anchor_pos: anchor_pos,
			anchor_ofs: anchor_ofs,
			base_scale: (ELEMENT_BASE_SCALE, ELEMENT_BASE_SCALE),			
			cur_scale: [0.0, 0.0, 0.0],
			cur_position: [0.0, 0.0, 0.0],	
			mouse_input_handler: HandlerOption::None,
			keyboard_input_handler: HandlerOption::None,
		}
	}

	pub fn mouse_input_handler(mut self, mouse_input_handler: MouseInputHandler) -> UiElement {
		self.mouse_input_handler = HandlerOption::Fn(mouse_input_handler);
		self
	}

	pub fn keyboard_input_handler(mut self, keyboard_input_handler: KeyboardInputHandler) -> UiElement {
		if let HandlerOption::None = self.keyboard_input_handler {
				self.keyboard_input_handler = HandlerOption::Fn(keyboard_input_handler);
				self
		} else {
			panic!("UiElement::keyboard_input_handler(): Keyboard input already assigned \
				to: '{:?}'", self.keyboard_input_handler);
		}
	}

	pub fn sub(mut self, mut sub_element: UiElement, handles_keyb: bool) -> UiElement {
		sub_element.anchor_pos[2] += window::SUBDEPTH;
		self.sub_elements.reserve_exact(1);

		if handles_keyb {
			if let HandlerOption::None = self.keyboard_input_handler {
				let next_sub_ele_idx = self.sub_elements.len();
				self.keyboard_input_handler = HandlerOption::Sub(next_sub_ele_idx);
			} else {
				panic!("UiElement::sub(): Cannot assign a sub-element to handle keyboard \
					input if it has already been assigned. Current assignment: '{:?}'."
					, self.keyboard_input_handler);
			}
		}
		
		self.sub_elements.push(sub_element);
		self
	}

	pub fn text_string(mut self, text_string: &str) -> UiElement {
		self.text.string = text_string.to_string();
		self
	}

	pub fn text_color(mut self, color: (f32, f32, f32, f32)) -> UiElement {
		self.text.color = color;
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

	pub fn draw_text<S: Surface>(&self, text_system: &TextSystem, target: &mut S,
				font_texture: &FontTexture) 
	{
		let text_display = TextDisplay::new(text_system, font_texture, 
			self.get_text());

		glium_text::draw(&text_display, text_system, target, 
			self.text_matrix(), self.text().get_color());

		for element in self.sub_elements.iter() {
			element.draw_text(text_system, target, font_texture);
		}
	}

	pub fn set_text_width(&mut self, ts: &TextSystem, ft: &FontTexture) {
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

	pub fn text(&self) -> &TextProperties {
		&self.text
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

	pub fn set_keybd_focus(&mut self, has_focus: bool) {
		self.has_keybd_focus = has_focus;
	}

	// [FIXME]: Unused Vars.
	#[allow(unused_variables)]
	pub fn handle_mouse_input(&mut self, state: ElementState, button: MouseButton, 
				window: &mut MainWindow) -> MouseInputEventResult 
	{
		if let HandlerOption::Fn(ref mut mih) = self.mouse_input_handler {
			mih(state, button, window)
		} else {
			MouseInputEventResult::None
		}
	}

	// [FIXME]: Unused Vars.
	// [FIXME]: Error message (set up result type).
	#[allow(unused_variables)]
	pub fn handle_keyboard_input(&mut self, state: ElementState, vk_code: Option<VirtualKeyCode>, 
				window: &mut MainWindow) -> KeyboardInputEventResult 
	{
		let result = match self.keyboard_input_handler {
			HandlerOption::Fn(ref mut kih) => kih(state, vk_code, window),
			HandlerOption::Sub(ele_idx) => {
				assert!(ele_idx < self.sub_elements.len(), "{}UiElement::handle_keyboard_input(): {}:{}",
					module_path!(), column!(), line!());
				print!("        Passing keyboard input, '{:?}::{:?}', to sub element '{}' --->", 
					state, vk_code, ele_idx);
				self.sub_elements[ele_idx].handle_keyboard_input(state, vk_code, window);
				KeyboardInputEventResult::None
			},
			_ => KeyboardInputEventResult::None,
		};

		match result {
			KeyboardInputEventResult::AppendCharacterToTextString(c) => {
				println!("        KeyboardInputEventResult: {}", c);
				self.text.string.push(c);
			},
			_ => ()
		}

		result
	}


	///////// [FIXME]: CACHE THIS STUFF PROPERLY ////////// 
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


// Ensure position is within -1.0 and 1.0 for x and y dims.
fn verify_position(position: [f32; 3]) {
	assert!((position[0] <= 1.0 && position[0] >= -1.0) 
			|| (position[1] <= 1.0 && position[1] >= -1.0), 
		format!("UiElement::new(): Position out of range: [x: {}, y: {}, z:{}]. \
			'x' and 'y' must both be between -1.0 and 1.0.", 
			position[0], position[1], position[2])
	);
}
