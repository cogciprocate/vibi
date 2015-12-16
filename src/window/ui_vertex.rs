
#[derive(Copy, Clone)]
pub struct UiVertex {
	position: [f32; 3],
	color: [f32; 3],
	normal: [f32; 3],
}

impl UiVertex {
	#[allow(dead_code)]
	pub fn new(position: [f32; 3], color: [f32; 3], normal: [f32; 3]) -> UiVertex {
		UiVertex { position: position, color: color, normal: normal }
	}

	// TODO: DEPRICATE
	#[allow(dead_code)]
	pub fn shifted(position: [f32; 3], color: [f32; 3], normal: [f32; 3], shift_by: &[f32; 3]
			) -> UiVertex 
	{
		let shifted_position = shift(&position, shift_by);

		UiVertex::new(shifted_position, color, normal)
	}

	// TODO: DEPRICATE
	#[allow(dead_code)]
	pub fn shift(&self, shift_by: &[f32; 3]) -> UiVertex {
		UiVertex::shifted(self.position, self.color, self.normal, shift_by)
	}

	// TODO: Convert to taking a matrix argument.
	pub fn transform(&self, scale_by: &[f32; 3], shift_by: &[f32; 3]) -> UiVertex {
		let position = shift(&scale(&self.position, scale_by), shift_by);
		// let position = shift(&self.position, shift_by);
		UiVertex { position: position, ..self.clone() }
	}

	pub fn set_color(&mut self, color: [f32; 3]) {
		self.color = color;
	}
}

implement_vertex!(UiVertex, position, color, normal);


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
