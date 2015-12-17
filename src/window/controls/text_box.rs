// use super::{};
use super::DEFAULT_SCALE;
use window::{shapes, UiElement};

pub struct TextBox;

impl TextBox {
	pub fn new(anchor_pos: [f32; 3], offset: (f32, f32), extra_width: f32,
			label: String, color: [f32; 3]) -> UiElement
	{
		// // NOTE: width(x): 1.15470053838 (2/sqrt(3)), height(y): 1.0
		// let ew = extra_width;
		// let a = 0.5;
		// let s = 0.57735026919; // 1/sqrt(3)
		// let hs = s * 0.5;

		// let vertices = vec![
		// 	UiVertex::new([ 0.0, 		 0.0, 	 0.0], color, [0.0, 0.0, -1.0]),
		// 	UiVertex::new([-(hs + ew),	 a,  	 0.0], color, [0.0, 0.0, -1.0]),
		// 	UiVertex::new([ hs + ew, 	 a,  	 0.0], color, [0.0, 0.0, -1.0]),
		// 	UiVertex::new([ s + ew, 	 0.0,  	 0.0], color, [0.0, 0.0, -1.0]),
		// 	UiVertex::new([ hs + ew, 	-a, 	 0.0], color, [0.0, 0.0, -1.0]),
		// 	UiVertex::new([-(hs + ew), 	-a,  	 0.0], color, [0.0, 0.0, -1.0]),
		// 	UiVertex::new([-(s + ew),  	 0.0,  	 0.0], color, [0.0, 0.0, -1.0]),
		// ];

		// let indices = vec![
		// 	0, 1, 2,
		// 	2, 3, 0,
		// 	0, 3, 4,
		// 	4, 5, 0,
		// 	0, 5, 6,
		// 	6, 1, 0u16,
		// ];

		// // Distance from center to edges for mouse focus detection purposes:
		// let radii = (ew + (s * 0.75), a);

		let (vertices, indices, radii) = shapes::hexagon_panel(1.0, extra_width, color);

		UiElement::new(anchor_pos, [offset.0, offset.1, 0.0], (DEFAULT_SCALE, DEFAULT_SCALE), 
				vertices, indices, label, radii)
			.text_offset(((-extra_width / 2.0) - 1.2, 0.0))
	}

}
