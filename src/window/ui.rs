#![allow(dead_code, unused_variables)]
use glium::backend::glutin_backend::{ GlutinFacade };
use glium::{ self, VertexBuffer, IndexBuffer, Program, DrawParameters, Surface, };
use glium::vertex::{ EmptyInstanceAttributes as EIAttribs };
use super::{ UiVertex, UiElement };

const TWOSR3: f32 = 1.15470053838;

pub struct Ui<'d> {
	vbo: Option<VertexBuffer<UiVertex>>,
	ibo: Option<IndexBuffer<u16>>,
	elements: Vec<UiElement>,
	program: Program,
	params: DrawParameters<'d>,
	display: &'d GlutinFacade,
	scale: f32,
	// perspective: [[f32; 4]; 4],
	// models: Vec<([[f32; 4]; 4], [f32; 3])>,
}

impl<'d> Ui<'d> {
	pub fn new(display: &'d GlutinFacade) -> Ui<'d> {
		let scale = 1.0;
		let vbo = None;
		let ibo = None;

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

		// let perspective = [
		// 	[ ar, 0.0, 0.0, 0.0],
		// 	[0.0, 1.0, 0.0, 0.0],
		// 	[0.0, 0.0, 1.0, 1.0],
		// 	[0.0, 0.0, 0.0, 1.0f32]
		// ];

		// let mut models = Vec::new();

		// models.push((
		// 	[ 	[0.1, 0.0, 0.0, 0.0],
		// 	  	[0.0, 0.1, 0.0, 0.0],
		// 	  	[0.0, 0.0, 1.0, 0.0],
		// 	  	[1.0, -1.0 + (0.1), 0.0, 1.0f32], ],
		// 	super::C_ORANGE
		// ));

		Ui { 
			vbo: vbo,
			ibo: ibo,
			elements: Vec::new(),
			program: program,
			params: params,
			display: display,
			scale: scale,
			// perspective: perspective,
			// models: models,
		}
	}

	pub fn element(mut self, element: UiElement) -> Ui<'d> {
		if self.vbo.is_some() || self.ibo.is_some() { 
			panic!("Ui::element(): [FIXME]: Cannot (yet) add element after initialization.") 
		}

		self.elements.push(element);
		self
	}

	pub fn init(mut self) -> Ui<'d> {
		let mut vertices: Vec<UiVertex> = Vec::new();
		let mut indices: Vec<u16> = Vec::new();

		for element in self.elements.iter() {
			indices.extend_from_slice(&element.indices(vertices.len() as u16));

			vertices.extend_from_slice(&element.vertices(
				self.display.get_framebuffer_dimensions(), self.scale
			));
		}

		self.vbo = Some(VertexBuffer::dynamic(self.display, &vertices).unwrap());
		self.ibo = Some(IndexBuffer::new(self.display, glium::index::PrimitiveType::TrianglesList, 
			&indices).unwrap());

		self
	}

	pub fn resize(&mut self) {
		match self.vbo {
			Some(ref mut vbo) => {
				let mut vertices: Vec<UiVertex> = Vec::with_capacity(vbo.len());

				for element in self.elements.iter() {
					vertices.extend_from_slice(&element.vertices(
						self.display.get_framebuffer_dimensions(), self.scale
					));
				}

				vbo.write(&vertices);
			},

			None => panic!("Ui::resize(): Cannot resize until Ui has been \
				initialized with .init()"),
		}
	}

	pub fn draw<S: Surface>(&self, target: &mut S) {
		if self.vbo.is_none() || self.ibo.is_none() { 
			panic!("Ui::draw(): Buffers not initialized.") 
		}

		// Get frame dimensions:
		let (width, height) = target.get_dimensions();

		let ar = height as f32 / width as f32;	

		let perspective = [
			[ ar, 0.0, 0.0, 0.0],
			[0.0, 1.0, 0.0, 0.0],
			[0.0, 0.0, 1.0, 1.0],
			[0.0, 0.0, 0.0, 1.0f32]
		];
		// let perspective = [
		// 	[1.0, 0.0, 0.0, 0.0],
		// 	[0.0, 1.0, 0.0, 0.0],
		// 	[0.0, 0.0, 1.0, 1.0],
		// 	[0.0, 0.0, 0.0, 1.0f32]
		// ];

		// let model = 
		// 	[ 	[0.1, 0.0, 0.0, 0.0],
		// 	  	[0.0, 0.1, 0.0, 0.0],
		// 	  	[0.0, 0.0, 1.0, 0.0],
		// 	  	[(1.0 - (TWOSR3 * 0.05)) / ar, -1.0 + (0.1), 0.0, 1.0f32], ];

		// let model_color = [1.0, 1.0, 1.0f32];
		let model_color = super::C_ORANGE;

		// Uniforms:
		let uniforms = uniform! {		
			// model: model,
			// view: view,
			persp: perspective,
			// u_light_pos: light_pos,
			u_model_color: model_color,
			// grid_side: self.grid_side,
			// diffuse_tex: &diffuse_texture,
			// normal_tex: &normal_map,
		};		

		// Draw Grid:
		target.draw((self.vbo.as_ref().unwrap(), EIAttribs { len: 1 }), self.ibo.as_ref().unwrap(), 
			&self.program, &uniforms, &self.params).unwrap();
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
	// out vec3 v_color;
	// out vec3 v_normal;	

	// uniform uint grid_side;
	// uniform mat4 model;
	// uniform mat4 view;
	uniform mat4 persp;

	void main() {
		// gl_Position = vec4(position, 1.0);
		gl_Position = persp /** model*/ * vec4(position, 1.0);

		// v_normal = transpose(inverse(mat3(model_view))) * normal;
		// v_color = color;
		// v_position = gl_Position.xyz / gl_Position.w;
	};
"#;
		

// Fragment Shader:
#[allow(non_upper_case_globals)]
static fragment_shader_src: &'static str = r#"
	#version 330

	// in vec3 v_color; // <-- currently unused (using uniform atm)
	// in vec3 v_normal;
	// in vec3 v_position;

	out vec4 color;

	// uniform vec3 u_light_pos;
	uniform vec3 u_model_color;

	// const float ambient_strength = 0.1;
	// const vec3 ambient_color = vec3(0.9, 0.9, 0.9);
	// const vec3 diffuse_color = vec3(0.2, 0.2, 0.2);
	// const vec3 specular_color = vec3(0.4, 0.4, 0.4);
	// const float specular_coeff = 16.0;

	// // Pastel orange:
	// const vec3 model_color = vec3(0.9607, 0.4745, 0.0);
	// // Pink model:
	// const vec3 model_color = vec3(0.9882, 0.4902, 0.7059);

	void main() {
		// float diffuse_ampl = max(dot(normalize(v_normal), normalize(u_light_pos)), 0.0);

		// vec3 camera_dir = normalize(-v_position);
		// vec3 half_direction = normalize(normalize(u_light_pos) + camera_dir);
		// float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 
		// 	specular_coeff);

		// color = vec4((ambient_color * u_model_color) + diffuse_ampl
		// 	* diffuse_color + specular * specular_color, 1.0);	

		color = vec4(u_model_color, 1.0);
	};
"#;


// fn persp_matrix(width: u32, height: u32, /*zoom: f32*/) -> [[f32; 4]; 4] {
// 	// let zfar = 1024.0;
// 	// let znear = 0.1;

// 	// let (width, height) = target.get_dimensions();
// 	let aspect_ratio = height as f32 / width as f32;
// 	// let fov: f32 = 3.141592 / zoom;	
// 	// let f = 1.0 / (fov / 2.0).tan();

// 	// [
// 	// 	[f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
// 	// 	[         0.0         ,     f ,              0.0              ,   0.0],
// 	// 	[         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
// 	// 	[         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
// 	// ]

// 	[
// 		[	   aspect_ratio   ,    0.0,              0.0              ,   0.0],
// 		[         0.0         ,    1.0,              0.0              ,   0.0],
// 		[         0.0         ,    0.0,  			1.0    			,   1.0],
// 		[         0.0         ,    0.0, 			0.0 			,   1.0f32],
// 	]
// }


// fn vbo(display: &GlutinFacade) -> VertexBuffer<Vertex> {
// 	// NOTE: width(x): 1.15470053838 (2/sqrt(3)), height(y): 1.0
// 	let a = 0.5;
// 	let s = 0.57735026919; // 1/sqrt(3)
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


// fn ibo(display: &GlutinFacade) -> IndexBuffer<u16> {
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

