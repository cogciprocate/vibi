
use window::{ui_shape_2d, UiElement};

pub struct HexButton;

impl HexButton {
	pub fn new(anchor_pos: [f32; 3], offset: (f32, f32), extra_width: f32,
			text: &str, color: [f32; 4]) -> UiElement
	{
		// let (vertices, indices, radii) = ui_shape::hexagon_panel(1.0, extra_width, 0.0, color);
		let shape = ui_shape_2d::hexagon_panel(1.0, extra_width, 0.0, color);

		// println!("    Hexagon perimeter edges list: {:?}", shape.perim_edges());
		// shape.as_border(0.1, color);

		UiElement::new(anchor_pos, [offset.0, offset.1, 0.0], shape)
			.text_string(text)
	}
}
