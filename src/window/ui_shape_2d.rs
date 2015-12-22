#[allow(dead_code)]

use window::{self, UiVertex};

#[derive(Clone)]
pub struct UiShape2d {
	pub vertices: Vec<UiVertex>,
	pub indices: Vec<u16>,
	pub radii: (f32, f32),
	pub color: [f32; 4],
}

impl UiShape2d {

}


pub fn hexagon_panel(height: f32, ew: f32, depth: f32, color: [f32; 4], 
		// ) -> (Vec<UiVertex>, Vec<u16>, (f32, f32)) 
		) -> UiShape2d
{
	// NOTE: width(x): 1.15470053838 (2/sqrt(3)), height(y): 1.0
	let sqrt_3_inv = 1.732050808;

	let a = height / 2.0;
	let s = 1.0 / sqrt_3_inv; // 1/sqrt(3)
	let hs = s * 0.5;

	let vertices = vec![
		UiVertex::new([ 0.0, 		 0.0, 	 depth], color, [0.0, 0.0], false),
		UiVertex::new([-(hs + ew),	 a,  	 depth], color, [0.0, 0.0], true),
		UiVertex::new([ hs + ew, 	 a,  	 depth], color, [0.0, 0.0], true),
		UiVertex::new([ s + ew, 	 0.0,  	 depth], color, [0.0, 0.0], true),
		UiVertex::new([ hs + ew, 	-a, 	 depth], color, [0.0, 0.0], true),
		UiVertex::new([-(hs + ew), 	-a,  	 depth], color, [0.0, 0.0], true),
		UiVertex::new([-(s + ew),  	 0.0,  	 depth], color, [0.0, 0.0], true),
	];

	let indices = vec![
		0, 1, 2,
		2, 3, 0,
		0, 3, 4,
		4, 5, 0,
		0, 5, 6,
		6, 1, 0u16,
	];

	let radii = (ew + (s * 0.75), a);

	// (vertices, indices, radii)
	UiShape2d { vertices: vertices, indices: indices, radii: radii, color: window::C_BLACK }	
}


pub fn rectangle(height: f32, width: f32, depth: f32, color: [f32; 4], 
		// ) -> (Vec<UiVertex>, Vec<u16>, (f32, f32)) 
		) -> UiShape2d
{
	let top = height / 2.0;
	let bot = -height / 2.0;
	let left = -width / 2.0;
	let right = width / 2.0;

	let xy_normal = [0.0, 0.0];

	let vertices = vec![
		UiVertex::new([ 0.0, 	 0.0, 	 depth], color, xy_normal, false),
		UiVertex::new([ left, 	 top, 	 depth], color, xy_normal, true),
		UiVertex::new([ right, 	 top, 	 depth], color, xy_normal, true),
		UiVertex::new([ right, 	 bot, 	 depth], color, xy_normal, true),
		UiVertex::new([ left, 	 bot, 	 depth], color, xy_normal, true),
	];

	// println!("\n\n##### Rectangle vertices: {:?}\n", vertices[0].position());

	let indices = vec![
		0, 1, 2,
		2, 3, 0,
		0, 3, 4,
		4, 1, 0,
	];

	let radii = (right, top);

	// (vertices, indices, radii)
	UiShape2d { vertices: vertices, indices: indices, radii: radii, color: window::C_BLACK }	
}
