#![allow(dead_code, unused_variables)]
use glium::backend::glutin_backend::{ GlutinFacade };
use glium::{ self, Surface, Program, DrawParameters, VertexBuffer, IndexBuffer };

const GRID_SIDE: u32 = 64;
const MAX_GRID_SIDE: u32 = 8192;
const HEX_X: f32 = 0.086602540378 + 0.01;
const HEX_Y: f32 = 0.05 + 0.01;


pub struct HexGrid<'d> {
	vertices: VertexBuffer<Vertex>,
	indices: IndexBuffer<u16>,
	program: Program,
	params: DrawParameters<'d>,
	pub grid_side: u32,
}

impl<'d> HexGrid<'d> {
	pub fn new(display: &GlutinFacade) -> HexGrid {
		// The greatest hexagon ever made (o rly?):
		let vertices = hex_vbo(display);
		let indices = hex_ibo(display);

		// Create program:
		let program = Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

		// Draw parameters:
		let params = DrawParameters {
			depth: glium::Depth {
				test: glium::DepthTest::IfLess,
				write: true,
				.. Default::default()
			},
			// backface_culling: glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
			backface_culling: glium::draw_parameters::BackfaceCullingMode::CullingDisabled, // <-- default
			.. Default::default()
		};

		HexGrid {
			vertices: vertices,
			indices: indices,
			program: program,
			params: params,
			grid_side: GRID_SIDE,
		}
	}

	pub fn draw<S: Surface>(&self, target: &mut S, elapsed_ms: f64) {
		// Set up our frame-countery-thing:
		let f_c = (elapsed_ms / 4000.0) as f32;

		// Get frame dimensions:
		let (width, height) = target.get_dimensions();

		// Center of hex grid:
		let grid_ctr_x = HEX_X * (self.grid_side as f32 - 1.0);
		let grid_top_y = (HEX_Y * (self.grid_side as f32 - 1.0)) / 2.0;
		let grid_ctr_z = -grid_ctr_x * 1.5;

		// Grid count:
		// // Grow and shrink grid count:
		// let ii = i / 1000;
		// let grid_count = if (ii / GRID_COUNT) & 1 == 1 {
		// 	GRID_COUNT - (ii % GRID_COUNT) } else { (ii % GRID_COUNT) };
		let grid_count = (self.grid_side * self.grid_side) as usize;	

		// Perspective transformation matrix:
		let persp = persp_matrix(width, height, 3.0);

		// Camera position:
		let cam_x = f32::cos(f_c) * grid_ctr_x * 0.8;
		let cam_y = f32::cos(f_c) * grid_top_y * 0.8;
		let cam_z = f32::cos(f_c / 3.0) * grid_ctr_z * 0.4; // <-- last arg sets zoom range

		// View transformation matrix: { position(x,y,z), direction(x,y,z), up_dim(x,y,z)}
		let view = view_matrix(
			&[	grid_ctr_x + cam_x, 
				0.0 + cam_y, 
				(grid_ctr_z * 0.4) + cam_z + -1.7],  // <-- second f32 sets z base
			&[	0.0 - (cam_x / 5.0), 
				0.0 - (cam_y / 5.0), 
				0.5 * -grid_ctr_z],  // <-- first f32 sets distant focus point
			&[0.0, 1.0, 0.0]
		);

		// Model transformation matrix:
		let grid_model = [
			[1.0, 0.0, 0.0, 0.0],
			[0.0, 1.0, 0.0, 0.0],
			[0.0, 0.0, 1.0, 0.0],
			[0.0, 0.0, 0.0, 1.0f32]
		];

		// Light position:
		let light_pos = [-1.0, 0.4, -0.9f32];

		// Model color:
		let model_color = [
			(f32::abs(f32::cos(f_c / 3.0) * 0.99)) + 0.001, 
			(f32::abs(f32::sin(f_c / 2.0) * 0.99)) + 0.001, 
			(f32::abs(f32::cos(f_c / 1.0) * 0.99)) + 0.001,
		];

		// Uniforms:
		let uniforms = uniform! {		
			model: grid_model,
			view: view,
			persp: persp,
			u_light_pos: light_pos,
			u_model_color: model_color,
			grid_side: self.grid_side,
			// diffuse_tex: &diffuse_texture,
			// normal_tex: &normal_map,
		};

		// Draw Grid:
		target.draw((&self.vertices, glium::vertex::EmptyInstanceAttributes { 
			len: grid_count }), &self.indices, &self.program, &uniforms, 
			&self.params).unwrap();
	}
}



// Vertex Shader:
#[allow(non_upper_case_globals)]
static vertex_shader_src: &'static str = r#"
	#version 330

	in vec3 position;
	in vec3 color;
	in vec3 normal;

	out vec3 v_position;
	out vec3 v_color;
	out vec3 v_normal;	

	uniform uint grid_side;
	uniform mat4 model;
	uniform mat4 view;
	uniform mat4 persp;

	void main() {

		float border = 0.01;

		float x_scl = 0.086602540378f + border;
		float y_scl = 0.05 + border;

		float u_id = float(uint(gl_InstanceID) % grid_side);
		float v_id = float(uint(gl_InstanceID) / grid_side);

		float x_pos = ((v_id + u_id) * x_scl) + position.x;
		float y_pos = ((v_id * -y_scl) + (u_id * y_scl)) + position.y;

		mat4 model_view = view * model;

		gl_Position = persp * model_view * vec4(x_pos, y_pos, 0.0, 1.0);
		v_normal = transpose(inverse(mat3(model_view))) * normal;
		v_color = color;
		v_position = gl_Position.xyz / gl_Position.w;
	};
"#;
		

// Fragment Shader:
#[allow(non_upper_case_globals)]
static fragment_shader_src: &'static str = r#"
	#version 330

	in vec3 v_color; // <-- currently unused (using uniform atm)
	in vec3 v_normal;
	in vec3 v_position;

	out vec4 color;

	uniform vec3 u_light_pos;
	uniform vec3 u_model_color;

	// const float ambient_strength = 0.1;
	const vec3 ambient_color = vec3(0.9, 0.9, 0.9);
	const vec3 diffuse_color = vec3(0.2, 0.2, 0.2);
	const vec3 specular_color = vec3(0.4, 0.4, 0.4);
	const float specular_coeff = 16.0;

	// // Pastel orange:
	// const vec3 model_color = vec3(0.9607, 0.4745, 0.0);
	// // Pink model:
	// const vec3 model_color = vec3(0.9882, 0.4902, 0.7059);

	void main() {
		float diffuse_ampl = max(dot(normalize(v_normal), normalize(u_light_pos)), 0.0);

		vec3 camera_dir = normalize(-v_position);
		vec3 half_direction = normalize(normalize(u_light_pos) + camera_dir);
		float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 
			specular_coeff);

		color = vec4((ambient_color * u_model_color) + diffuse_ampl
			* diffuse_color + specular * specular_color, 1.0);	
	};
"#;



fn hex_vbo(display: &GlutinFacade) -> glium::vertex::VertexBuffer<Vertex> {
	let a = 0.5 / 10.0f32;
	let s = 0.57735026919 / 10.0f32; // 1/sqrt(3)
	let hs = s / 2.0f32;

	glium::vertex::VertexBuffer::new(display, &[
			Vertex::new([ 0.0, 	 0.0, 	 0.0], [0.4, 0.4, 0.4,], [0.0, 0.0, -1.0]),
			Vertex::new([-hs, 	 a,  	 0.0], [0.7, 0.7, 0.2,], [0.0, 0.0, -1.0]),
			Vertex::new([ hs, 	 a,  	 0.0], [0.2, 0.7, 0.7,], [0.0, 0.0, -1.0]),
			Vertex::new([ s, 	 0.0,  	 0.0], [0.7, 0.2, 0.7,], [0.0, 0.0, -1.0]),
			Vertex::new([ hs, 	-a, 	 0.0], [0.7, 0.7, 0.2,], [0.0, 0.0, -1.0]),
			Vertex::new([-hs, 	-a,  	 0.0], [0.2, 0.7, 0.7,], [0.0, 0.0, -1.0]),
			Vertex::new([-s, 	 0.0,  	 0.0], [0.7, 0.2, 0.7,], [0.0, 0.0, -1.0]),
		]).unwrap()
}


fn hex_ibo(display: &GlutinFacade) -> glium::IndexBuffer<u16> {
	glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &[
			0, 1, 2,
			2, 3, 0,
			0, 3, 4,
			4, 5, 0,
			0, 5, 6,
			6, 1, 0u16,
		]).unwrap()
}


#[derive(Copy, Clone)]
struct Vertex {
	position: [f32; 3],
	color: [f32; 3],
	normal: [f32; 3],
}

impl Vertex {
	fn new(position: [f32; 3], color: [f32; 3], normal: [f32; 3]) -> Vertex {
		Vertex { position: position, color: color, normal: normal }
	}
}
implement_vertex!(Vertex, position, color, normal);



fn persp_matrix(width: u32, height: u32, zoom: f32) -> [[f32; 4]; 4] {
	let zfar = 1024.0;
	let znear = 0.1;

	// let (width, height) = target.get_dimensions();
	let aspect_ratio = height as f32 / width as f32;
	let fov: f32 = 3.141592 / zoom;	
	let f = 1.0 / (fov / 2.0).tan();

	[
		[f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
		[         0.0         ,     f ,              0.0              ,   0.0],
		[         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
		[         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
	]
}



fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
	let f = {
		let f = direction;
		let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
		let len = len.sqrt();
		[f[0] / len, f[1] / len, f[2] / len]
	};

	let s = [up[1] * f[2] - up[2] * f[1],
			 up[2] * f[0] - up[0] * f[2],
			 up[0] * f[1] - up[1] * f[0]];

	let s_norm = {
		let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
		let len = len.sqrt();
		[s[0] / len, s[1] / len, s[2] / len]
	};

	let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
			 f[2] * s_norm[0] - f[0] * s_norm[2],
			 f[0] * s_norm[1] - f[1] * s_norm[0]];

	let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
			 -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
			 -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

	[
		[s[0], u[0], f[0], 0.0],
		[s[1], u[1], f[1], 0.0],
		[s[2], u[2], f[2], 0.0],
		[p[0], p[1], p[2], 1.0],
	]
}


