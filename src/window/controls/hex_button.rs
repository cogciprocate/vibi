
use window::{shapes, UiElement};

pub struct HexButton;

impl HexButton {
	pub fn new(anchor_pos: [f32; 3], offset: (f32, f32), extra_width: f32,
			text: &str, color: [f32; 3]) -> UiElement
	{
		let (vertices, indices, radii) = shapes::hexagon_panel(1.0, extra_width, 0.0, color);

		UiElement::new(anchor_pos, [offset.0, offset.1, 0.0], vertices, indices, radii)
			.text_string(text)
	}
}
