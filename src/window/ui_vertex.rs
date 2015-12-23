
// [FIXME]: TODO: 
// - Seriously revamp this a fix all the extra allocations etc.
//    - ^ kinda halfway done...
#[derive(Copy, Clone, Debug)]
pub struct UiVertex {
	position: [f32; 3],
	color: [f32; 4],
	xy_normal: [f32; 2],
	is_perimeter: bool,
}

impl UiVertex {
	#[allow(dead_code)]
	pub fn new(position: [f32; 3], color: [f32; 4], xy_normal: [f32; 2], is_perimeter: bool) 
			-> UiVertex 
	{
		UiVertex { position: position, color: color, xy_normal: xy_normal, is_perimeter: is_perimeter }
	}

	// // TODO: DEPRICATE
	// #[allow(dead_code)]
	// pub fn shifted(position: [f32; 3], color: [f32; 3], xy_normal: [f32; 3], shift_by: &[f32; 3]
	// 		) -> UiVertex 
	// {
	// 	let shifted_position = shift(&position, shift_by);

	// 	UiVertex::new(shifted_position, color, xy_normal)
	// }

	pub fn scale(mut self, scale_by: &[f32; 3]) -> UiVertex {
		// UiVertex { position: scale(&self.position, scale_by), color: self.color, xy_normal: self.xy_normal }
		self.position = scale(&self.position, scale_by);
		self
	}

	pub fn shift(mut self, shift_by: &[f32; 3]) -> UiVertex {
		// UiVertex::shifted(self.position, self.color, self.xy_normal, shift_by)
		self.position = shift(&self.position, shift_by);
		self
	}	

	#[allow(dead_code)]
	pub fn color(mut self, color: [f32; 4]) -> UiVertex {
		self.color = color;
		self
	}

	// TODO: Convert to taking a matrix argument.
	pub fn transform(self, scale_by: &[f32; 3], shift_by: &[f32; 3]) -> UiVertex {
		// let position = shift(&scale(&self.position, scale_by), shift_by);
		// // let position = shift(&self.position, shift_by);
		// UiVertex { position: position, ..self.clone() }
		self.scale(scale_by).shift(shift_by)
	}

	// pub fn border_of(mut self, thickness: f32) -> UiVertex {
	// 	self.position[0] = rim_job(self.position[0], thickness);
	// 	self.position[1] = rim_job(self.position[1], thickness);
	// 	self
	// }

	#[allow(dead_code)]
	pub fn set_color(&mut self, color: [f32; 4]) {
		self.color = color;
	}

	#[allow(dead_code)]
	pub fn position(&self) -> &[f32; 3] {
		&self.position
	}

	#[allow(dead_code)]
	pub fn is_perimeter(&self) -> bool {
		self.is_perimeter
	}
}

implement_vertex!(UiVertex, position, color, xy_normal);


// fn rim_job(coord: f32, thickness: f32) -> f32 {
// 	if coord > 0.0 {
// 		coord + thickness
// 	} else if coord < 0.0 {
// 		coord - thickness
// 	} else {
// 		coord
// 	}
// }

// TODO: Combine into transform().
fn shift(position: &[f32; 3], shift_by: &[f32; 3]) -> [f32; 3] {
	[
		position[0] + shift_by[0],
		position[1] + shift_by[1],
		position[2] + shift_by[2],
	]
}

// TODO: Combine into transform().
fn scale(position: &[f32; 3], scale_by: &[f32; 3]) -> [f32; 3] {
	[
		position[0] * scale_by[0],
		position[1] * scale_by[1],
		position[2] * scale_by[2],
	]
}
