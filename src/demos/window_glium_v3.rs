#![allow(unused_imports, unused_variables, unused_mut)]
use std::thread;
use std::time::{ Duration };
use std::io::{ Cursor };
use std::sync::mpsc::{ Receiver, Sender };
use glium::{ self, DisplayBuild, Surface };
use image;
use find_folder::{ Search };

// use interactive as iact;
use cyc_loop::{ CyCtl, CySts };
use teapot;


#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, normal, tex_coords);


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


// Vertex Shader:
#[allow(non_upper_case_globals)]
static vertex_shader_src: &'static str = r#"
    #version 330

    in vec3 position;
    in vec3 normal;
    in vec2 tex_coords;

    out vec3 v_normal;
    out vec3 v_position;
    out vec2 v_tex_coords;

    uniform mat4 perspective;
    uniform mat4 view;
    uniform mat4 model;

    void main() {
    	v_tex_coords = tex_coords;
    	mat4 model_view = view * model;
    	v_normal = transpose(inverse(mat3(model_view))) * normal;
        gl_Position = perspective * model_view * vec4(position, 1.0);
        v_position = gl_Position.xyz / gl_Position.w;
    }
"#;

// Fragment Shader:
#[allow(non_upper_case_globals)]
static fragment_shader_src: &'static str = r#"
	#version 330
	in vec3 v_normal;
	in vec3 v_position;
	in vec2 v_tex_coords;
	out vec4 color;
	uniform vec3 u_light;
	uniform sampler2D diffuse_tex;
	uniform sampler2D normal_tex;
	const vec3 specular_color = vec3(1.0, 1.0, 1.0);

	mat3 cotangent_frame(vec3 normal, vec3 pos, vec2 uv) {
	    vec3 dp1 = dFdx(pos);
	    vec3 dp2 = dFdy(pos);
	    vec2 duv1 = dFdx(uv);
	    vec2 duv2 = dFdy(uv);
	    vec3 dp2perp = cross(dp2, normal);
	    vec3 dp1perp = cross(normal, dp1);
	    vec3 T = dp2perp * duv1.x + dp1perp * duv2.x;
	    vec3 B = dp2perp * duv1.y + dp1perp * duv2.y;
	    float invmax = inversesqrt(max(dot(T, T), dot(B, B)));
	    return mat3(T * invmax, B * invmax, normal);
	}

	void main() {
	    vec3 diffuse_color = texture(diffuse_tex, v_tex_coords).rgb;
	    vec3 ambient_color = diffuse_color * 0.1;
	    vec3 normal_map = texture(normal_tex, v_tex_coords).rgb;
	    mat3 tbn = cotangent_frame(v_normal, v_position, v_tex_coords);
	    vec3 real_normal = normalize(tbn * -(normal_map * 2.0 - 1.0));
	    float diffuse = max(dot(real_normal, normalize(u_light)), 0.0);
	    vec3 camera_dir = normalize(-v_position);
	    vec3 half_direction = normalize(normalize(u_light) + camera_dir);
	    float specular = pow(max(dot(half_direction, real_normal), 0.0), 16.0);
	    color = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, 1.0);
	}
"#;


pub fn window(control_tx: Sender<CyCtl>, status_rx: Receiver<CySts>) {
  	let display = glium::glutin::WindowBuilder::new()
  		.with_depth_buffer(24)
  		.build_glium().unwrap();

  	// Define teapot:
	// let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
	// let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
	// let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
	// 	&teapot::INDICES).unwrap();

  	// Define square:
	let shape = glium::vertex::VertexBuffer::new(&display, &[
            Vertex { position: [-1.0,  1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 1.0] },
            Vertex { position: [ 1.0,  1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 1.0] },
            Vertex { position: [-1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 0.0] },
            Vertex { position: [ 1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 0.0] },
        ]).unwrap();	

	// Create program:
	let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

	// Load image:
	// let assets = Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
	// let rust_logo_path = assets.join("rust.png");
	let image = image::load(Cursor::new(&include_bytes!(
		"/home/nick/projects/vibi/assets/tuto-14-diffuse.jpg")[..]), image::JPEG).unwrap().to_rgba();
	let image_dimensions = image.dimensions();
	let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
	let diffuse_texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

	let image = image::load(Cursor::new(&include_bytes!(
		"/home/nick/projects/vibi/assets/tuto-14-normal.png")[..]), image::PNG).unwrap().to_rgba();
	let image_dimensions = image.dimensions();
	let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
	let normal_map = glium::texture::Texture2d::new(&display, image).unwrap();

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
	
	// Elapsed time:
	let mut t: f32 = -0.5;

	// Event/Rendering loop:
  	loop {
	    // listing the events produced by the window and waiting to be received
	    for ev in display.poll_events() {
	        match ev {
	            glium::glutin::Event::Closed => {
	            	// control_tx.send(CyCtl::Exit).expect("Exit button control tx");
	            	return;
            	},   // the window has been closed by the user
	            _ => ()
	        }
	    }
  		
	    let mut target = display.draw();
	    target.clear_color_and_depth((0.2, 0.2, 1.0, 1.0), 1.0);

	    // Perspective transformation matrix:
		let mut perspective = {
		    let (width, height) = target.get_dimensions();
		    let aspect_ratio = height as f32 / width as f32;

		    let fov: f32 = 3.141592 / 3.0;
		    let zfar = 1024.0;
		    let znear = 0.1;

		    let f = 1.0 / (fov / 2.0).tan();

		    [
		        [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
		        [         0.0         ,     f ,              0.0              ,   0.0],
		        [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
		        [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
		    ]
		};

		// View transformation matrix: { position(x,y,z), direction(x,y,z), up_dim(x,y,z)}
		let view = view_matrix(&[0.5, 0.5, -2.0], &[-0.5, -0.5, 2.0], &[0.0, 1.0, 0.0]);

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
			perspective: perspective,
			u_light: light,
			diffuse_tex: &diffuse_texture,
			normal_tex: &normal_map,
		};

		// target.draw((&positions, &normals), &indices, &program, &uniforms, &params).unwrap();
		target.draw(&shape, glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip), &program,
			&uniforms, &params).unwrap();
		// target.draw(&shape, glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip), &program,
  //                   &uniform! { model: model, view: view, perspective: perspective,
  //                               u_light: light, diffuse_tex: &diffuse_texture, normal_tex: &normal_map },
  //                   &params).unwrap();

	    target.finish().unwrap();
	    t += 0.0002;
	}


}

