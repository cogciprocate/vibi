// use super::{};
use window::{shapes, UiElement};

pub struct TextBox;

impl TextBox {
	pub fn new(anchor_pos: [f32; 3], offset: (f32, f32), extra_width: f32,
			label: String, color: [f32; 3]) -> UiElement
	{
		//	(vertices, indices, radii)
		let hex = shapes::hexagon_panel(1.0, extra_width, 0.0, color);
		// let ind_shift = hex.0.len() as u16;

		// let rect = shapes::rectangle(0.8, extra_width, window::SUBDEPTH, [1.0, 1.0, 1.0]);

		// let vertices = hex.0.into_iter().chain(rect.0.into_iter()).collect();
		// let indices = hex.1.into_iter().chain(rect.1.into_iter().map(|ind| ind + ind_shift)).collect();
		// let vertices = rect.0.clone();
		// let indices = rect.1.clone();

		UiElement::new(anchor_pos, [offset.0, offset.1, 0.0], hex.0, hex.1, hex.2)
			.text(label)
			.text_offset(((-extra_width / 2.0) - 1.2, 0.0))
			.sub(TextField::new(anchor_pos, offset, extra_width))
	}

}


pub struct TextField;

impl TextField {
	pub fn new(anchor_pos: [f32; 3], offset: (f32, f32), width: f32) -> UiElement
	{
		//	(vertices, indices, radii)
		let rect = shapes::rectangle(0.8, width, -0.1, [1.0, 1.0, 1.0]);

		UiElement::new(anchor_pos, [offset.0, offset.1, 0.0], rect.0, rect.1, rect.2)
			.text_offset(((-width / 2.0) - 1.2, 0.0))
	}
}
