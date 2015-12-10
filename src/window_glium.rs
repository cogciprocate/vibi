#![allow(unused_imports, unused_variables, unused_mut)]
use std::thread;
use std::time::{ Duration };
use std::io::{ Cursor };
use std::sync::mpsc::{ Receiver, Sender };
use glium::{ self, DisplayBuild, Surface };
use glium::backend::glutin_backend::{ GlutinFacade };
use image;
use find_folder::{ Search };

// use interactive as iact;
use cyc_loop::{ CyCtl, CySts };
use teapot;


const GRID_SIDE: usize = 16;
const HEX_X: f32 = 0.086602540378 + 0.01;


// Vertex Shader:
#[allow(non_upper_case_globals)]
static vertex_shader_src: &'static str = r#"
	#version 330

	in vec3 position;
	in vec3 color;

	out vec3 v_color;

	uniform mat4 model;
	uniform mat4 view;
	uniform mat4 persp;

	void main() {
		v_color = color;

		uint grid_dim = 16;

		float border = 0.01;

		float x_scl = 0.086602540378f + border;
		float y_scl = 0.05 + border;

		float u = float(uint(gl_InstanceID) % grid_dim);
		float v = float(uint(gl_InstanceID) / grid_dim);

		float x_pos = ((v + u) * x_scl) + position.x;
		float y_pos = ((v * -y_scl) + (u * y_scl)) + position.y;

		gl_Position = persp * view * model * vec4(x_pos, y_pos, 0.0, 1.0);
	};
"#;
		

// Fragment Shader:
#[allow(non_upper_case_globals)]
static fragment_shader_src: &'static str = r#"
	#version 330

	in vec3 v_color;

	out vec4 color;

	uniform vec3 u_light;

	void main() {
		color = vec4(v_color, 1.0);
	};
"#;


pub fn window(control_tx: Sender<CyCtl>, status_rx: Receiver<CySts>) {
	let display = glium::glutin::WindowBuilder::new()
		.with_depth_buffer(24)
		.build_glium().unwrap();

	// Create the greatest hexagon ever made:
	let hex_vertices = hex_vbo(&display);
	let hex_indices = hex_ibo(&display);

	// Create program:
	let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

	// Depth param:
	let params = glium::DrawParameters {
		depth: glium::Depth {
			test: glium::DepthTest::IfLess,
			write: true,
			.. Default::default()
		},
		// // Use to avoid drawing the back side of triangles:
		// backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
		.. Default::default()
	};

	// Light direction:
	let light = [1.4, 0.4, 0.9f32];
	
	// Loop mutables:
	let mut t: f32 = -0.5;
	let mut exit_app: bool = false;

	// Event/Rendering loop:
	loop {
		for ev in display.poll_events() {
			use glium::glutin::Event::{ Closed, KeyboardInput };
			match ev {
				Closed => {					
					exit_app = true;
				},

				KeyboardInput(state, code, vk_code_o) => {
					use glium::glutin::ElementState::{ Released, Pressed };
					if let Released = state {
						if let Some(vk_code) = vk_code_o {
							use glium::glutin::VirtualKeyCode::{ Q, Escape };
							match vk_code {
								Q | Escape => exit_app = true,
								_ => (),
							}
						}
					}
				},

				_ => ()
			}
		}
		
		// Create draw target and clear color and depth:
		let mut target = display.draw();
		target.clear_color_and_depth((0.025, 0.025, 0.025, 1.0), 1.0);

		// Perspective transformation matrix:
		let persp = persp_matrix(&target, 3.0);

		// Center of hex grid:
		let mid_x_ofs = HEX_X * (GRID_SIDE - 1) as f32;

		// View transformation matrix: { position(x,y,z), direction(x,y,z), up_dim(x,y,z)}
		let view = view_matrix(
			&[mid_x_ofs, 0.0, -2.0], 
			&[0.0, 0.0, 2.0], 
			&[0.0, 1.0, 0.0]
		);

		// Model transformation matrix:
		let model = [
			[1.0, 0.0, 0.0, 0.0],
			[0.0, 1.0, 0.0, 0.0],
			[0.0, 0.0, 1.0, 0.0],
			[0.0, 0.0, 0.0, 1.0f32]
		];

		// Uniforms:
		let mut uniforms = uniform! {		
			model: model,
			view: view,
			persp: persp,
			u_light: light,
			// diffuse_tex: &diffuse_texture,
			// normal_tex: &normal_map,
		};

		// Draw:
		target.draw((&hex_vertices, glium::vertex::EmptyInstanceAttributes { 
			len: GRID_SIDE * GRID_SIDE }), &hex_indices, &program, &uniforms, 
			&params).unwrap();

		// Swap buffers:
		target.finish().unwrap();

		// Increment our 'counter':
		t += 0.0002;

		if exit_app {
			// control_tx.send(CyCtl::Exit).expect("Exit button control tx");
			break;
		}
	}
}


fn persp_matrix(target: &glium::Frame, zoom: f32) -> [[f32; 4]; 4] {
	let zfar = 1024.0;
	let znear = 0.1;

	let (width, height) = target.get_dimensions();
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


fn hex_vbo(display: &GlutinFacade) -> glium::vertex::VertexBuffer<Vertex> 
{
	let a = 0.5 / 10.0f32;
	let s = 0.57735026919 / 10.0f32; // 1/sqrt(3)
	let hs = s / 2.0f32;

	glium::vertex::VertexBuffer::new(display, &[
			Vertex::new([ 0.0, 	 0.0, 	 0.0], [0.7, 0.7, 0.7]),
			Vertex::new([-hs, 	 a,  	 0.0], [0.7, 0.7, 0.2,]),
			Vertex::new([ hs, 	 a,  	 0.0], [0.2, 0.7, 0.7,]),
			Vertex::new([ s, 	 0.0,  	 0.0], [0.7, 0.2, 0.7,]),
			Vertex::new([ hs, 	-a, 	 0.0], [0.7, 0.7, 0.2,]),
			Vertex::new([-hs, 	-a,  	 0.0], [0.2, 0.7, 0.7,]),
			Vertex::new([-s, 	 0.0,  	 0.0], [0.7, 0.2, 0.7,]),
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
}

impl Vertex {
	fn new(position: [f32; 3], color: [f32; 3]) -> Vertex {
		Vertex { position: position, color: color }
	}
}
implement_vertex!(Vertex, position, color);



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
