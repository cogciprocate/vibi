#[allow(dead_code)]

use window::UiVertex;


pub fn hexagon_panel(height: f32, ew: f32, depth: f32, color: [f32; 3], 
		) -> (Vec<UiVertex>, Vec<u16>, (f32, f32)) 
{
	// NOTE: width(x): 1.15470053838 (2/sqrt(3)), height(y): 1.0
	let sqrt_3 = 1.732050808;

	let a = height / 2.0;
	let s = 1.0 / sqrt_3; // 1/sqrt(3)
	let hs = s * 0.5;

	let vertices = vec![
		UiVertex::new([ 0.0, 		 0.0, 	 depth], color, [0.0, 0.0, -1.0]),
		UiVertex::new([-(hs + ew),	 a,  	 depth], color, [0.0, 0.0, -1.0]),
		UiVertex::new([ hs + ew, 	 a,  	 depth], color, [0.0, 0.0, -1.0]),
		UiVertex::new([ s + ew, 	 0.0,  	 depth], color, [0.0, 0.0, -1.0]),
		UiVertex::new([ hs + ew, 	-a, 	 depth], color, [0.0, 0.0, -1.0]),
		UiVertex::new([-(hs + ew), 	-a,  	 depth], color, [0.0, 0.0, -1.0]),
		UiVertex::new([-(s + ew),  	 0.0,  	 depth], color, [0.0, 0.0, -1.0]),
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

	(vertices, indices, radii)
}


pub fn rectangle(height: f32, width: f32, depth: f32, color: [f32; 3], 
		) -> (Vec<UiVertex>, Vec<u16>, (f32, f32)) 
{
	let top = height / 2.0;
	let bot = -height / 2.0;
	let left = -width / 2.0;
	let right = width / 2.0;

	let normal = [0.0, 0.0, -1.0];

	let vertices = vec![
		UiVertex::new([ 0.0, 	 0.0, 	 depth], color, normal),
		UiVertex::new([ left, 	 top, 	 depth], color, normal),
		UiVertex::new([ right, 	 top, 	 depth], color, normal),
		UiVertex::new([ right, 	 bot, 	 depth], color, normal),
		UiVertex::new([ left, 	 bot, 	 depth], color, normal),
	];

	// println!("\n\n##### Rectangle vertices: {:?}\n", vertices[0].position());

	let indices = vec![
		0, 1, 2,
		2, 3, 0,
		0, 3, 4,
		4, 1, 0,
	];

	let radii = (right, top);

	(vertices, indices, radii)
}
