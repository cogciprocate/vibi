#![allow(unused_imports, unused_variables, unused_mut, dead_code, unused_assignments)]
use std::f32;
use std::thread;
// use std::time::{ Duration as StdDuration };
use time::{self, Timespec, Duration};
use std::io::{Cursor};
use std::sync::mpsc::{Receiver, Sender};
use glium::{self, DisplayBuild, Surface};
use glium::backend::glutin_backend::{GlutinFacade};
use image;
use find_folder::{Search};
use glium_text;
use glium::glutin;
// use piston_window::{ PistonWindow, WindowSettings, };

// use interactive as iact;
use loop_cycles::{ CyCtl, CySts };
use super::{ WinStats, StatusText, HexGrid, Ui, UiElement };

const C_PINK: [f32; 3] = [0.9882, 0.4902, 0.7059];
const C_ORANGE: [f32; 3] = [0.9607, 0.4745, 0.0];

const GRID_SIDE: u32 = 64;
const MAX_GRID_SIDE: u32 = 8192;
const HEX_X: f32 = 0.086602540378 + 0.01;
const HEX_Y: f32 = 0.05 + 0.01;


pub fn open(control_tx: Sender<CyCtl>, status_rx: Receiver<CySts>) {	
	// Create our window:
	let display = glium::glutin::WindowBuilder::new()
		.with_depth_buffer(24)
		.with_dimensions(1400, 800)
		.with_title("Vibi".to_string())
		.with_multisampling(8)
		// .with_gl_robustness(glium::glutin::Robustness::NoError) // <-- Disabled for development
		.with_vsync()
		// .with_transparency(true)
		// .with_fullscreen(glium::glutin::get_primary_monitor())
		.build_glium().unwrap();

	// Hex grid:
	let mut hex_grid = HexGrid::new(&display);

	// Status text UI element (fps & grid side):
	let status_text = StatusText::new(&display);

	// Primary user interface:
	let mut ui = Ui::new(&display)
		// .element(UiElement::hex_button([anchor: x, y, z], (offset: x, y), scale, extra_width, text))
		// .element(UiElement::hex_button([1.0, 1.0, 0.0], (-0.06, -0.06), 0.06, 0.0, "yo")
		// .element(UiElement::hex_button([-1.0, -1.0, 0.0], (0.06, 0.06), 0.06, 0.0, "yo")
		.element(UiElement::hex_button([1.0, -1.0, 0.0], (-0.52, 0.06), 0.06, 2.0, "Settings".to_string()))
		.element(UiElement::hex_button([1.0, -1.0, 0.0], (-0.18, 0.06), 0.06, 2.0, "Exit".to_string()))
		.init();

	
	// Loop vars:
	let mut cycle_status = CySts::new();
	let mut stats = WinStats::new();
	let mut close_window: bool = false;

	// Print some stuff:
	println!("\t==== Vibi Experimental Window ====\n\
		\tPress 'Escape' or 'Q' to quit.\n\
		\tPress 'Up Arrow' to double or 'Down Arrow' to halve grid size.\n\
		\tPress 'Right Arrow' to increase or 'Left Arrow' to decrease grid size by one.");

	// Event/Rendering loop:
	loop {
		// Check cycle status:
		loop {
			match status_rx.try_recv() {
				Ok(cs) => {
					cycle_status = cs;
				},
				Err(_) => break,
			};
		}

		// Check input events:
		for ev in display.poll_events() {
			use glium::glutin::Event::{ Closed, KeyboardInput, Resized };
			match ev {
				Closed => {					
					close_window = true;
				},

				Resized(..) => {
					ui.resize()
				},

				KeyboardInput(state, code, vk_code_o) => {
					use glium::glutin::ElementState::{ Released, Pressed };
					if let Released = state {
						if let Some(vk_code) = vk_code_o {
							use glium::glutin::VirtualKeyCode::{ Q, Escape, Up, Down, Left, Right };
							match vk_code {
								Q | Escape => close_window = true,
								Up => if hex_grid.grid_side < MAX_GRID_SIDE { hex_grid.grid_side *= 2; },
								Down => if hex_grid.grid_side >= 4 { hex_grid.grid_side /= 2; },
								Right => if hex_grid.grid_side < MAX_GRID_SIDE { hex_grid.grid_side += 1; },
								Left => if hex_grid.grid_side > 2 { hex_grid.grid_side -= 1; },
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
		target.clear_color_and_depth((0.030, 0.050, 0.080, 1.0), 1.0);

		// Draw hex grid:
		hex_grid.draw(&mut target, stats.elapsed_ms());

		// Draw FPS and grid side text:
		status_text.draw(&mut target, &stats, hex_grid.grid_side);

		// Draw UI:
		ui.draw(&mut target);

		// Swap buffers:
		target.finish().unwrap();

		// Increment our counters:
		stats.incr();

		// Clean up and exit if necessary:
		if close_window {
			// control_tx.send(CyCtl::Exit).expect("Exit button control tx");
			break;
		}
	}
}



// fn persp_matrix(width: u32, height: u32, zoom: f32) -> [[f32; 4]; 4] {
// 	let zfar = 1024.0;
// 	let znear = 0.1;

// 	// let (width, height) = target.get_dimensions();
// 	let aspect_ratio = height as f32 / width as f32;
// 	let fov: f32 = 3.141592 / zoom;	
// 	let f = 1.0 / (fov / 2.0).tan();

// 	[
// 		[f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
// 		[         0.0         ,     f ,              0.0              ,   0.0],
// 		[         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
// 		[         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
// 	]
// }


// fn hex_vbo(display: &GlutinFacade) -> glium::vertex::VertexBuffer<Vertex> {
// 	let a = 0.5 / 10.0f32;
// 	let s = 0.57735026919 / 10.0f32; // 1/sqrt(3)
// 	let hs = s / 2.0f32;

// 	glium::vertex::VertexBuffer::new(display, &[
// 			Vertex::new([ 0.0, 	 0.0, 	 0.0], [0.4, 0.4, 0.4,], [0.0, 0.0, -1.0]),
// 			Vertex::new([-hs, 	 a,  	 0.0], [0.7, 0.7, 0.2,], [0.0, 0.0, -1.0]),
// 			Vertex::new([ hs, 	 a,  	 0.0], [0.2, 0.7, 0.7,], [0.0, 0.0, -1.0]),
// 			Vertex::new([ s, 	 0.0,  	 0.0], [0.7, 0.2, 0.7,], [0.0, 0.0, -1.0]),
// 			Vertex::new([ hs, 	-a, 	 0.0], [0.7, 0.7, 0.2,], [0.0, 0.0, -1.0]),
// 			Vertex::new([-hs, 	-a,  	 0.0], [0.2, 0.7, 0.7,], [0.0, 0.0, -1.0]),
// 			Vertex::new([-s, 	 0.0,  	 0.0], [0.7, 0.2, 0.7,], [0.0, 0.0, -1.0]),
// 		]).unwrap()
// }


// fn hex_ibo(display: &GlutinFacade) -> glium::IndexBuffer<u16> {
// 	glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &[
// 			0, 1, 2,
// 			2, 3, 0,
// 			0, 3, 4,
// 			4, 5, 0,
// 			0, 5, 6,
// 			6, 1, 0u16,
// 		]).unwrap()
// }


// #[derive(Copy, Clone)]
// struct Vertex {
// 	position: [f32; 3],
// 	color: [f32; 3],
// 	normal: [f32; 3],
// }

// impl Vertex {
// 	fn new(position: [f32; 3], color: [f32; 3], normal: [f32; 3]) -> Vertex {
// 		Vertex { position: position, color: color, normal: normal }
// 	}
// }
// implement_vertex!(Vertex, position, color, normal);



// fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
// 	let f = {
// 		let f = direction;
// 		let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
// 		let len = len.sqrt();
// 		[f[0] / len, f[1] / len, f[2] / len]
// 	};

// 	let s = [up[1] * f[2] - up[2] * f[1],
// 			 up[2] * f[0] - up[0] * f[2],
// 			 up[0] * f[1] - up[1] * f[0]];

// 	let s_norm = {
// 		let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
// 		let len = len.sqrt();
// 		[s[0] / len, s[1] / len, s[2] / len]
// 	};

// 	let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
// 			 f[2] * s_norm[0] - f[0] * s_norm[2],
// 			 f[0] * s_norm[1] - f[1] * s_norm[0]];

// 	let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
// 			 -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
// 			 -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

// 	[
// 		[s[0], u[0], f[0], 0.0],
// 		[s[1], u[1], f[1], 0.0],
// 		[s[2], u[2], f[2], 0.0],
// 		[p[0], p[1], p[2], 1.0],
// 	]
// }



// // Vertex Shader:
// #[allow(non_upper_case_globals)]
// static vertex_shader_src: &'static str = r#"
// 	#version 330

// 	in vec3 position;
// 	in vec3 color;
// 	in vec3 normal;

// 	out vec3 v_color;
// 	out vec3 v_normal;
// 	out vec3 v_position;

// 	uniform uint grid_side;
// 	uniform mat4 model;
// 	uniform mat4 view;
// 	uniform mat4 persp;

// 	void main() {

// 		float border = 0.01;

// 		float x_scl = 0.086602540378f + border;
// 		float y_scl = 0.05 + border;

// 		float u_id = float(uint(gl_InstanceID) % grid_side);
// 		float v_id = float(uint(gl_InstanceID) / grid_side);

// 		float x_pos = ((v_id + u_id) * x_scl) + position.x;
// 		float y_pos = ((v_id * -y_scl) + (u_id * y_scl)) + position.y;

// 		mat4 model_view = view * model;

// 		gl_Position = persp * model_view * vec4(x_pos, y_pos, 0.0, 1.0);
// 		v_normal = transpose(inverse(mat3(model_view))) * normal;
// 		v_color = color;
// 		v_position = gl_Position.xyz / gl_Position.w;
// 	};
// "#;
		

// // Fragment Shader:
// #[allow(non_upper_case_globals)]
// static fragment_shader_src: &'static str = r#"
// 	#version 330

// 	in vec3 v_color; // <-- currently unused (using uniform atm)
// 	in vec3 v_normal;
// 	in vec3 v_position;

// 	out vec4 color;

// 	uniform vec3 u_light_pos;
// 	uniform vec3 u_model_color;

// 	// const float ambient_strength = 0.1;
// 	const vec3 ambient_color = vec3(0.9, 0.9, 0.9);
// 	const vec3 diffuse_color = vec3(0.2, 0.2, 0.2);
// 	const vec3 specular_color = vec3(0.4, 0.4, 0.4);
// 	const float specular_coeff = 16.0;

// 	// // Pastel orange:
// 	// const vec3 model_color = vec3(0.9607, 0.4745, 0.0);
// 	// // Pink model:
// 	// const vec3 model_color = vec3(0.9882, 0.4902, 0.7059);

// 	void main() {
// 		float diffuse_ampl = max(dot(normalize(v_normal), normalize(u_light_pos)), 0.0);

// 		vec3 camera_dir = normalize(-v_position);
// 		vec3 half_direction = normalize(normalize(u_light_pos) + camera_dir);
// 		float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 
// 			specular_coeff);

// 		color = vec4((ambient_color * u_model_color) + diffuse_ampl
// 			* diffuse_color + specular * specular_color, 1.0);	
// 	};
// "#;
