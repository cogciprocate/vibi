#![allow(unused_imports, unused_variables)]
use std::thread;
use std::time::{ Duration };
use std::io::{ Cursor };
use std::sync::mpsc::{ Receiver, Sender };
use glium::{ self, DisplayBuild, Surface };
use image;
use find_folder::{ Search };

// use interactive as iact;
use cyc_loop::{ CyCtl, CyStatus };
use teapot;


#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);


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

    out vec3 v_normal;
    out vec3 v_position;

    uniform mat4 perspective;
    uniform mat4 view;
    uniform mat4 model;

    void main() {
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

    out vec4 color;

    uniform vec3 u_light;

    const vec3 ambient_color = vec3(0.2, 0.0, 0.0);
    const vec3 diffuse_color = vec3(0.6, 0.0, 0.0);
    const vec3 specular_color = vec3(1.0, 1.0, 1.0);

    void main() {
        // float brightness = dot(normalize(v_normal), normalize(u_light));
        // vec3 dark_color = vec3(0.6, 0.0, 0.0);
        // vec3 regular_color = vec3(1.0, 0.0, 0.0);
        // color = vec4(mix(dark_color, regular_color, brightness), 1.0);

        float diffuse = max(dot(normalize(v_normal), normalize(u_light)), 0.0);

        vec3 camera_dir = normalize(-v_position);
        vec3 half_direction = normalize(normalize(u_light) + camera_dir);
        float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);

        color = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, 1.0);        
    }
"#;


pub fn window(control_tx: Sender<CyCtl>, status_rx: Receiver<CyStatus>) {
    #![allow(unused_mut)]
      let display = glium::glutin::WindowBuilder::new()
          .with_depth_buffer(24)
          .build_glium().unwrap();

    // Define teapot:
    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
        &teapot::INDICES).unwrap();

    // Create program:
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    // Load image:
    // let assets = Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    // let rust_logo_path = assets.join("rust.png");
    let image = image::load(Cursor::new(&include_bytes!("/home/nick/projects/vibi/assets/opengl.png")[..]),
        image::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);

    // Create image texture: 
    let texture = glium::texture::Texture2d::new(&display, image).unwrap();

    // Light direction:
    let light = [-1.0, 0.4, 0.9f32];

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
        let view = view_matrix(&[-0.5, 1.0, -2.0], &[0.5, -1.0, 2.0], &[0.0, 1.0, 0.0]);

        // Model transformation matrix:
        let model = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ];

        // Uniforms:
        let mut uniforms = uniform! {        
            model: model,
            view: view,
            perspective: perspective,
            u_light: light,        
        };

        target.draw((&positions, &normals), &indices, &program, &uniforms, &params).unwrap();

        target.finish().unwrap();
        t += 0.0002;
    }


}



// // Empty 'uniforms' and default 'parameters': 
// target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
//     &Default::default()).unwrap();


// Uniforms:
// #[allow(unused_mut)]
// let mut uniforms = uniform! {        
//     // // Shifts model position with t:
//     // matrix: [
//     //     [1.0, 0.0, 0.0, 0.0],
//     //     [0.0, 1.0, 0.0, 0.0],
//     //     [0.0, 0.0, 1.0, 0.0],
//     //     [ t , 0.0, 0.0, 1.0f32],
//     // ]
//     // // Rotates model clockwise with t:
//     // matrix: [
//     //     [ t.cos(), t.sin(), 0.0, 0.0],
//     //     [-t.sin(), t.cos(), 0.0, 0.0],
//     //     [0.0, 0.0, 1.0, 0.0],
//     //     [0.0, 0.0, 0.0, 1.0f32],
//     // ],
//     // matrix: [
//     //     [0.01, 0.0, 0.0, 0.0],
//     //     [0.0, 0.01, 0.0, 0.0],
//     //     [0.0, 0.0, 0.01, 0.0],
//     //     [0.0, 0.0, 2.0, 1.0f32]
//     // ],
//     model: model,
//     // tex: &texture,
//     perspective: perspective,
//     u_light: light,        
// };


// // Vertex Shader:
// #[allow(non_upper_case_globals)]
// static vertex_shader_src: &'static str = r#"
//     #version 330

//     // in vec2 position;
//     // out vec2 my_attr;
//     in vec3 position;
//     in vec3 normal;

//     // in vec2 tex_coords;
//     // out vec2 v_tex_coords;

//     // uniform float t;
//     uniform mat4 matrix;

//     void main() {
//         // my_attr = position;
//         // v_tex_coords = tex_coords;
//         gl_Position = matrix * vec4(position, 1.0);
//     }
// "#;

// // Fragment Shader:
// #[allow(non_upper_case_globals)]
// static fragment_shader_src: &'static str = r#"
//     #version 330

//     // in vec2 my_attr;
//     in vec2 v_tex_coords;
//     out vec4 color;

//     uniform sampler2D tex;

//     void main() {
//         color = vec4(0.5, 0.4, 0.3, 1.0);
//         // color = vec4(my_attr, 1.0, 1.0);
//         // color = texture(tex, v_tex_coords);
//     }
// "#;
