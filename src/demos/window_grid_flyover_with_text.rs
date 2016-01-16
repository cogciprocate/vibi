#![allow(unused_imports, unused_variables, unused_mut, dead_code, unused_assignments)]
use std::f32;
use std::thread;
// use std::time::{ Duration as StdDuration };
use time::{ self, Timespec, Duration };
use std::io::{ Cursor };
use std::sync::mpsc::{ Receiver, Sender };
use glium::{ self, DisplayBuild, Surface };
use glium::backend::glutin_backend::{ GlutinFacade };
use image;
use find_folder::{ Search };
use glium_text;
use glium::glutin;

// use interactive as iact;
use loop_cycles::{ CyCtl, CyStatus };
use window_stats::{ WindowStats };

const C_PINK: [f32; 3] = [0.9882, 0.4902, 0.7059];
const C_ORANGE: [f32; 3] = [0.9607, 0.4745, 0.0];

const GRID_SIDE: u32 = 64;
const MAX_GRID_SIDE: u32 = 8192;
const HEX_X: f32 = 0.086602540378 + 0.01;
const HEX_Y: f32 = 0.05 + 0.01;


// Vertex Shader:
#[allow(non_upper_case_globals)]
static vertex_shader_src: &'static str = r#"
	#version 330

	in vec3 position;
	in vec3 color;
	in vec3 normal;

	out vec3 v_color;
	out vec3 v_normal;
	out vec3 v_position;

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
	const vec3 ambient_color = vec3(0.6, 0.6, 0.6);
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


pub fn window(control_tx: Sender<CyCtl>, status_rx: Receiver<CyStatus>) {	
	// Create our window:
	let display = glium::glutin::WindowBuilder::new()
		.with_depth_buffer(24)
		.with_dimensions(1200, 800)
		.with_title("Vibi".to_string())
		.with_multisampling(8)
		.with_gl_robustness(glium::glutin::Robustness::NoError)
		// .with_vsync()
		// .with_transparency(true)
		// .with_fullscreen(glium::glutin::get_primary_monitor())
		.build_glium().unwrap();


	//////////////////////////////////////////////////////////////////////
	//////////////////////////////// TEXT ////////////////////////////////
	//////////////////////////////////////////////////////////////////////

	// Text system (experimental):
	let text_system = glium_text::TextSystem::new(&display);

	// Font:
	let font_size = 24;
	let font = glium_text::FontTexture::new(&display, &include_bytes!(
			"/home/nick/projects/vibi/assets/fonts/NotoSans/NotoSans-Regular.ttf"
		)[..], font_size).unwrap();

	//////////////////////////////////////////////////////////////////////
	//////////////////////////////////////////////////////////////////////


	// Create the greatest hexagon ever made:
	let hex_vertices = hex_vbo(&display);
	let hex_indices = hex_ibo(&display);

	// Create program:
	let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

	// Draw parameters:
	let params = glium::DrawParameters {
		depth: glium::Depth {
			test: glium::DepthTest::IfLess,
			write: true,
			.. Default::default()
		},
		// backface_culling: glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
		.. Default::default()
	};	

	// Print random deets:
	println!("\t==== Vibi Experimental Window ====\n\
		\tPress 'Escape' or 'Q' to quit.\n\
		\tPress 'Up Arrow' to double or 'Down Arrow' to halve grid size.\n\
		\tPress 'Right Arrow' to increase or 'Left Arrow' to decrease grid size by one.");
	
	// Loop vars:
	// let mut i: usize = 0;
	let mut cycle_status = CyStatus::new();
	let mut f_c: f32 = -0.5;
	let mut stats = WindowStats::new();
	let mut exit_app: bool = false;
	let mut grid_side = GRID_SIDE;

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
			use glium::glutin::Event::{ Closed, KeyboardInput };
			match ev {
				Closed => {					
					exit_app = true;
				},

				KeyboardInput(state, code, vk_code_o) => {
					use glium::glutin::ElementState::{ Released, Pressed };
					if let Released = state {
						if let Some(vk_code) = vk_code_o {
							use glium::glutin::VirtualKeyCode::{ Q, Escape, Up, Down, Left, Right };
							match vk_code {
								Q | Escape => exit_app = true,
								Up => if grid_side < MAX_GRID_SIDE { grid_side *= 2; },
								Down => if grid_side > 1 { grid_side /= 2; },
								Right => if grid_side < MAX_GRID_SIDE { grid_side += 1; },
								Left => if grid_side > 1 { grid_side -= 1; },
								_ => (),
							}
						}
					}
				},

				_ => ()
			}
		}

		// Get frame dimensions 
		// [FIXME]: TODO: Is 'target.get_dimensions()' better to call?:
		let (width, height) = display.get_framebuffer_dimensions();

		// Create draw target and clear color and depth:
		let mut target = display.draw();
		target.clear_color_and_depth((0.030, 0.050, 0.080, 1.0), 1.0);


		// Center of hex grid:
		let grid_ctr_x = HEX_X * (grid_side as f32 - 1.0);
		let grid_top_y = (HEX_Y * (grid_side as f32 - 1.0)) / 2.0;
		let grid_ctr_z = -grid_ctr_x * 1.5;

		// Grid count:
		// // Grow and shrink grid count:
		// let ii = i / 1000;
		// let grid_count = if (ii / GRID_COUNT) & 1 == 1 {
		// 	GRID_COUNT - (ii % GRID_COUNT) } else { (ii % GRID_COUNT) };
		let grid_count = (grid_side * grid_side) as usize;	

		// Perspective transformation matrix:
		let persp = persp_matrix(width, height, 3.0);

		// Camera position:
		let cam_x = f32::cos(f_c) * grid_ctr_x * 0.8;
		let cam_y = f32::sin(f_c) * grid_top_y * 0.8;
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
			(f32::abs(f32::cos(f_c / 3.0) * 0.6)) + 0.1, 
			(f32::abs(f32::cos(f_c / 2.0) * 0.6)) + 0.1, 
			(f32::abs(f32::cos(f_c / 1.0) * 0.6)) + 0.1,
		];

		// Uniforms:
		let mut uniforms = uniform! {		
			model: grid_model,
			view: view,
			persp: persp,
			u_light_pos: light_pos,
			u_model_color: model_color,
			grid_side: grid_side,
			// diffuse_tex: &diffuse_texture,
			// normal_tex: &normal_map,
		};

		//////////////////////////////// TEXT /////////////////////////////////

		// let text_model_matrix = [
		// 	[2.0 / text_width, 0.0, 0.0, 0.0,],
		// 	[0.0, 2.0 * (width as f32) / (height as f32) / text_width, 0.0, 0.0,],
		// 	[0.0, 0.0, 1.0, 0.0,],
		// 	[-1.0, -1.0, 0.0, 1.0f32,],
		// ];

		
		let text_scl = 0.019; // / ((width * height) as f32 / 1000000.0);
		// let text_x_scl = text_scl * 2.0 / text_width;
		// let text_y_scl = text_scl * 2.0 * (width as f32) / (height as f32) / text_width;

		let text_x_scl = text_scl / (width as f32 / 1000.0);
		let text_y_scl = text_x_scl * (width as f32) / (height as f32);

		// FPS Text:
		let fps_text_matrix = [
			[text_x_scl, 0.0, 0.0, 0.0,],
			[0.0, text_y_scl, 0.0, 0.0,],
			[0.0, 0.0, 1.0, 0.0,],
			[-1.0, 1.0 - (2.0 * text_y_scl), 0.0, 1.0f32,],
		];
		let fps_text = glium_text::TextDisplay::new(&text_system, &font, 
			&format!("FPS: {}", stats.fps()));
		glium_text::draw(&fps_text, &text_system, &mut target, fps_text_matrix, 
			(0.99, 0.99, 0.99, 1.0));


		// Grid Side Text:
		let gs_text_matrix = [
			[text_x_scl, 0.0, 0.0, 0.0,],
			[0.0, text_y_scl, 0.0, 0.0,],
			[0.0, 0.0, 1.0, 0.0,],
			[-1.0, 1.0 - (4.0 * text_y_scl), 0.0, 1.0f32,],
		];
		let gs_text = glium_text::TextDisplay::new(&text_system, &font, 
			&format!("Grid: {gs} X {gs}", gs = grid_side));
		glium_text::draw(&gs_text, &text_system, &mut target, gs_text_matrix, 
			(0.99, 0.99, 0.99, 1.0));


		///////////////////////////////////////////////////////////////////////

		// Draw:
		target.draw((&hex_vertices, glium::vertex::EmptyInstanceAttributes { 
			len: grid_count }), &hex_indices, &program, &uniforms, 
			&params).unwrap();

		// Swap buffers:
		target.finish().unwrap();

		// Increment our counters:
		// i += 1;
		// f_c += 0.00350;
		f_c = (stats.elapsed_ms() / 4000.0) as f32;
		stats.incr();

		// Clean up and exit if necessary:
		if exit_app {
			// control_tx.send(CyCtl::Exit).expect("Exit button control tx");
			break;
		}
	}
}



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
