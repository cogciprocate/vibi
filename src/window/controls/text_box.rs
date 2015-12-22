// use super::{};
use window::{self, util, ui_shape_2d, UiElement, MouseInputEventResult, KeyboardInputEventResult};
use glium::glutin::{ElementState};

pub struct TextBox;

impl TextBox {
	pub fn new(anchor_pos: [f32; 3], offset: (f32, f32), extra_width: f32,
			label: &str, color: [f32; 4]) -> UiElement
	{
		//	(vertices, indices, radii)
		let shape = ui_shape_2d::hexagon_panel(1.0, extra_width, 0.0, color);
		let shape_c = shape.clone();

		UiElement::new(anchor_pos, [offset.0, offset.1, 0.0], shape_c.vertices, shape_c.indices, shape_c.radii, shape)
			.text_string(label)
			.text_offset(((-extra_width / 2.0) - 1.5, 0.0))
			.sub(TextField::new(anchor_pos, offset, extra_width), true)
			.mouse_input_handler(Box::new(|_, _, _| MouseInputEventResult::RequestKeyboardFocus(true)))
	}
}


pub struct TextField;

impl TextField {
	pub fn new(anchor_pos: [f32; 3], offset: (f32, f32), width: f32) -> UiElement
	{
		//	(vertices, indices, radii)
		let shape = ui_shape_2d::rectangle(0.8, width + 2.4, -0.1, [1.0, 1.0, 1.0, 1.0]);
		let shape_c = shape.clone();

		let new_offset = [
			offset.0 + 0.06,
			offset.1,
			0.0,
		];

		UiElement::new(anchor_pos, new_offset, shape_c.vertices, shape_c.indices, shape_c.radii, shape)
		.text_string("TextField")
		.text_offset((-(shape_c.radii).0 + 0.16, 0.0))
		.keyboard_input_handler(Box::new(|key_state, vk_code, kb_state, _| {
			if let ElementState::Pressed = key_state {
				use glium::glutin::VirtualKeyCode::*;
				match vk_code {
					Some(Back) => return KeyboardInputEventResult::PopTextString,

					_ => {
						if let Some(mut c) = util::map_vkc(vk_code) {					
							// println!("    VirtualKeyCode: {:?} => {:?}", vk_code, c);
							if kb_state.shift { c = c.to_uppercase().next().unwrap_or(c); }

							return KeyboardInputEventResult::PushTextString(c);
						}
					},
				}
			}

			KeyboardInputEventResult::None
		}))
		.border(0.05, window::C_BLACK)
	}
}
