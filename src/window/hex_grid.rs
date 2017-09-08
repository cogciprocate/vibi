#![allow(dead_code, unused_variables)]
// use std::sync::mpsc::Sender;
use glium::backend::glutin_backend::{GlutinFacade};
use glium::{self, Surface, Program, DrawParameters, VertexBuffer, IndexBuffer};
use glium::glutin::{ElementState, MouseButton};
// use vecmath;
use enamel::MouseState;
use bismit::flywheel::AreaInfo;
use bismit::map::SliceTractMap;
use window::HexGridBuffer;

const HEX_X: f32 = 0.086602540378 + 0.01;
const HEX_Y: f32 = 0.05 + 0.01;

const TEXT_SCALE: f32 = 0.018;
const TEXT_COLOR: (f32, f32, f32, f32) = (0.99, 0.99, 0.99, 1.0);


pub struct HexGrid<'d> {
    vertices: VertexBuffer<Vertex>,
    indices: IndexBuffer<u16>,
    program: Program,
    params: DrawParameters<'d>,
    pub cam_pos_norm: [f32; 3],
    pub cam_pos_raw: [f32; 3],
    model_plane_size: [f32; 2],
    viewable_plane_size: [f32; 2],
    pub buffer: HexGridBuffer,
    being_dragged: bool,
    surface_dims: (u32, u32),
    light_pos: [f32; 3],
    global_color: [f32; 3],
    pub top_right_scene: [f32; 4],
}

impl<'d> HexGrid<'d> {
    pub fn new(display: &GlutinFacade, area_info: AreaInfo) -> HexGrid {
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

        let buffer = HexGridBuffer::new(area_info, &display);
        let default_cam_dst = 1.5f32;

        let mut hg = HexGrid {
            vertices: vertices,
            indices: indices,
            program: program,
            params: params,
            cam_pos_norm: [0.0, 0.0, default_cam_dst],
            cam_pos_raw: [0.0, 0.0, -1.0],
            model_plane_size: [0.0, 0.0],
            viewable_plane_size: [0.0, 0.0],
            buffer: buffer,
            being_dragged: false,
            surface_dims: display.get_framebuffer_dimensions(),
            light_pos: [-1.0, 0.4, -0.9f32],
            global_color: [0.0, 0.0, 0.3f32],
            top_right_scene: [0.0; 4],
        };
        hg.update_cam_pos();
        hg
    }

    pub fn draw<S: Surface>(&mut self, target: &mut S, elapsed_ms: f64) {
        // Set up our frame-countery-thing:
        let f_c = (elapsed_ms * 0.00025) as f32;

        // Get frame dimensions:
        self.surface_dims = target.get_dimensions();

        // Perspective transformation matrix:
        let persp = persp_matrix(self.surface_dims.0, self.surface_dims.1, 3.0);

        // View transformation matrix: { position(x,y,z), direction(x,y,z), up_dim(x,y,z)}
        let view = view_matrix(&self.cam_pos_raw, &[0.0, 0.0, 1.0], &[0.0, 1.0, 0.0]);

        let slc_count = self.buffer.cur_slc_range().len();
        // self.model_plane_size = [0.0, 0.0];

        // Loop through currently visible slices:
        for slc_id in self.buffer.cur_slc_range().clone() {
            let grid_dims = self.buffer.tract_map().slc_dims(slc_id as u8);

            // let x_size = (grid_dims.0 + grid_dims.1) as f32 * HEX_X;
            let y_size = (grid_dims.0 + grid_dims.1) as f32 * HEX_Y;

            let scl = 100.0 / y_size;

            let slc_idm = self.buffer.cur_slc_range().end - 1;

            // Set up model position:
            // let x_shift = 18.0 * slc_count as f32 * (slc_idm - slc_id) as f32;
            // let y_shift = 10.0 * slc_count as f32 * (slc_idm - slc_id) as f32;
            let x_shift = 90.0 * (slc_idm - slc_id) as f32;
            let y_shift = 50.0 * (slc_idm - slc_id) as f32;
            let z_shift = 1.0;

            // Model transformation matrix:
            let model = [
                [scl, 0.0, 0.0, 0.0],
                [0.0, scl, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [x_shift, y_shift, z_shift, 1.0f32]
            ];

            // Uniforms:
            let uniforms = uniform! {
                model: model,
                view: view,
                persp: persp,
                u_light_pos: self.light_pos,
                u_global_color: self.global_color,
                grid_v_size: grid_dims.0,
                grid_u_size: grid_dims.1,
            };

            // Draw Grid (with per-instance vertex buffer):
            target.draw((&self.vertices, self.buffer.raw_states_buf(slc_id as u8)
                .per_instance().unwrap()), &self.indices, &self.program, &uniforms,
                &self.params).unwrap();
        }
    }

    // [TODO]: Simplify this mess and try to get it more accurate.
    pub fn update_cam_pos(&mut self) {
        let aspect_ratio = self.surface_dims.1 as f32 / self.surface_dims.0 as f32;
        let slc_count = self.buffer.cur_slc_range().len();

        let x_ofs = 75.0;
        let cam_x_pos = self.cam_pos_norm[0].mul_add(-1000.0, x_ofs);

        let y_ofs = -0.0;
        let cam_y_pos = self.cam_pos_norm[1].mul_add(1000.0, y_ofs);

        let z_ofs = -0.01;
        let cam_z_pos = self.cam_pos_norm[2].mul_add(-53.0, z_ofs);

        self.cam_pos_raw = [cam_x_pos, cam_y_pos, cam_z_pos];
        // println!("CAMERA POSITION: norm: {:?}, raw: {:?}", self.cam_pos_norm, self.cam_pos_raw);
    }

    pub fn move_camera(&mut self, delta: (i32, i32)) {
        let delta_x = delta.0 as f32 / self.surface_dims.0 as f32;
        let new_cam_x = self.cam_pos_norm[0] + delta_x;
        let new_x_valid = (new_cam_x >= -1.0 && new_cam_x <= 1.0) as i32 as f32;
        self.cam_pos_norm[0] = new_x_valid.mul_add(delta_x, self.cam_pos_norm[0]);

        let delta_y = delta.1 as f32 / self.surface_dims.1 as f32;
        let new_cam_y = self.cam_pos_norm[1] + delta_y;
        let new_y_valid = (new_cam_y >= -1.0 && new_cam_y <= 1.0) as i32 as f32;
        self.cam_pos_norm[1] = new_y_valid.mul_add(delta_y, self.cam_pos_norm[1]);

        self.update_cam_pos();
    }

    pub fn zoom_camera(&mut self, delta: f32) {
        let delta_z = delta * -0.001;
        let new_cam_z = self.cam_pos_norm[2] + delta_z;
        let new_z_valid = (new_cam_z >= 0.00 && new_cam_z <= 10.00) as i32 as f32;
        self.cam_pos_norm[2] = new_z_valid.mul_add(delta_z, self.cam_pos_norm[2]);

        self.update_cam_pos();
    }

    pub fn handle_mouse_input(&mut self, button_state: ElementState, button: MouseButton) {

    }

    pub fn handle_mouse_moved(&mut self, mouse_state: &MouseState) {

    }

    pub fn camera_pos(&self) -> [f32; 3] {
        self.cam_pos_norm
    }

    pub fn cam_pos_raw(&self) -> [f32; 3] {
        self.cam_pos_raw
    }

    pub fn tract_map(&self) -> &SliceTractMap {
        &self.buffer.tract_map()
    }
}


fn identity_4x4() -> [[f32; 4]; 4] {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0f32]
    ]
}


// Vertex Shader:
#[allow(non_upper_case_globals)]
static vertex_shader_src: &'static str = r#"
    #version 330

    in vec3 position;
    in vec3 color;
    in vec3 normal;
    in float state;
    // in uchar state;

    out vec3 v_position;
    out vec3 v_color;
    out vec3 v_normal;
    out float v_state;
    // out uchar state;

    uniform uint grid_v_size;
    uniform uint grid_u_size;
    // uniform uvec2 dims;
    uniform mat4 model;
    uniform mat4 view;
    uniform mat4 persp;

    void main() {

        float border = 0.01;

        float x_scl = 0.086602540378f + border;
        float y_scl = 0.05 + border;

        float v_id = float(uint(gl_InstanceID) / grid_u_size);
        float u_id = float(uint(gl_InstanceID) % grid_u_size);

        float x_pos = ((v_id + u_id) * x_scl) + position.x;
        float y_pos = ((v_id * -y_scl) + (u_id * y_scl)) + position.y;

        mat4 model_view = view * model;

        gl_Position = persp * model_view * vec4(x_pos, y_pos, 0.0, 1.0);
        v_normal = transpose(inverse(mat3(model_view))) * normal;
        v_color = color;
        v_position = gl_Position.xyz / gl_Position.w;
        v_state = state;
    };
"#;


// Fragment Shader:
#[allow(non_upper_case_globals)]
static fragment_shader_src: &'static str = r#"
    #version 330

    // Unused (using uniform atm):
    in vec3 v_color;
    in vec3 v_normal;
    in vec3 v_position;
    // Determines red component:
    in float v_state;
    // in uchar v_state;

    out vec4 color;

    uniform vec3 u_light_pos;
    uniform vec3 u_global_color;

    // const float ambient_strength = 0.1;
    const vec3 ambient_color = vec3(0.9, 0.9, 0.9);
    const vec3 diffuse_color = vec3(0.2, 0.2, 0.2);
    const vec3 specular_color = vec3(0.3, 0.3, 0.3);
    const float specular_coeff = 16.0;

    // // Pastel orange:
    // const vec3 global_color = vec3(0.9607, 0.4745, 0.0);
    // // Pink model:
    // const vec3 global_color = vec3(0.9882, 0.4902, 0.7059);

    // float gt(float x, float y) {
    //     return max(sign(x - y), 0.0);
    // }

    float gt_zero(float val) {
        return max(sign(val - 0.0002), 0.0);
    }

    void main() {
        float diffuse_ampl = max(dot(normalize(v_normal), normalize(u_light_pos)), 0.0);

        vec3 camera_dir = normalize(-v_position);
        vec3 half_direction = normalize(normalize(u_light_pos) + camera_dir);
        float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0),
            specular_coeff);

        float state_norm = v_state / 255.0;
        float state_compressed = state_norm * 0.5;
        float state_compressed_boosted = state_compressed + (0.5 * gt_zero(state_norm));
        // float state_compressed_boosted = gt_zero(state_norm);

        vec3 tile_color = vec3(
            state_compressed_boosted,
            u_global_color.g,
            u_global_color.b - (u_global_color.b * state_compressed_boosted)
        );

        color = vec4((ambient_color * tile_color) + diffuse_ampl
            * diffuse_color + specular * specular_color, 1.0);
    };
"#;

// MISC CRAP:
//
// The safer comparison follows:
//
// float f1 = 10.0;
// float f2 = f1 / 3;
// float f3 = f2 * 3.0;
// float delta = f1 - f3;
// bool bEqual = -0.0001 < delta && delta < 0.0001;


// [FIXME]: CONVERT TO TRIANGLE STRIPS
fn hex_vbo(display: &GlutinFacade) -> glium::vertex::VertexBuffer<Vertex> {
    let a = 0.5 / 10.0f32;
    let s = 0.57735026919 / 10.0f32; // 1/sqrt(3)
    let hs = s / 2.0f32;

    glium::vertex::VertexBuffer::new(display, &[
            Vertex::new([ 0.0,      0.0,      0.0], [0.4, 0.4, 0.4,], [0.0, 0.0, -1.0]),
            Vertex::new([-hs,      a,       0.0], [0.7, 0.7, 0.2,], [0.0, 0.0, -1.0]),
            Vertex::new([ hs,      a,       0.0], [0.2, 0.7, 0.7,], [0.0, 0.0, -1.0]),
            Vertex::new([ s,      0.0,       0.0], [0.7, 0.2, 0.7,], [0.0, 0.0, -1.0]),
            Vertex::new([ hs,     -a,      0.0], [0.7, 0.7, 0.2,], [0.0, 0.0, -1.0]),
            Vertex::new([-hs,     -a,       0.0], [0.2, 0.7, 0.7,], [0.0, 0.0, -1.0]),
            Vertex::new([-s,      0.0,       0.0], [0.7, 0.2, 0.7,], [0.0, 0.0, -1.0]),
        ]).unwrap()
}

// [FIXME]: CONVERT TO TRIANGLE STRIPS (as above)
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


/// Returns a column-major perspective matrix.
fn persp_matrix(width: u32, height: u32, fov_zoom: f32) -> [[f32; 4]; 4] {
    let zfar = 1024.0;
    let znear = 0.1;

    // let (width, height) = target.get_dimensions();
    let aspect_ratio = height as f32 / width as f32;
    let fov: f32 = 3.141592 / fov_zoom;
    let f = 1.0 / (fov / 2.0).tan();

    [
        [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
        [         0.0         ,     f ,              0.0              ,   0.0],
        [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
        [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
    ]
}

/// Returns a column-major view matrix.
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




//     pub fn draw_old<S: Surface>(&self, target: &mut S, elapsed_ms: f64,
//                 hex_grid_buf: &HexGridBuffer)
//     {
//         // [FIXME]: TEMPORARY:
//         let grid_dims = (67u32, 67u32);
//         debug_assert!(hex_grid_buf.vertex_buf().len() == (grid_dims.0 * grid_dims.1) as usize);

//         // Set up our frame-countery-thing:
//         let f_c = (elapsed_ms / 4000.0) as f32;

//         // Get frame dimensions:
//         let (width, height) = target.get_dimensions();

//         // Center of hex grid:
//         // [FIXME]: TODO: CENTER NEEDS TO BE COMPUTED PROPERLY USING BOTH DIMS
//         let grid_ctr_x = HEX_X * (grid_dims.1 as f32 - 1.0);
//         let grid_top_y = (HEX_Y * (grid_dims.1 as f32 - 1.0)) / 2.0;
//         let grid_ctr_z = -grid_ctr_x * 1.5;

//         // Grid count:
//         let grid_count = (grid_dims.0 * grid_dims.1) as usize;

//         // Perspective transformation matrix:
//         let persp = persp_matrix(width, height, 3.0);

//         // z scale factor:
//         let z_scl = 0.05;
//         // x and y scale factor:
//         let xy_scl = 0.2;

//         // Camera position:
//         let cam_x = f32::cos(f_c)  * grid_ctr_x * xy_scl;
//         let cam_y = f32::cos(f_c)  * grid_top_y * xy_scl;
//         let cam_z = f32::cos(f_c / 3.0)  * grid_ctr_z * z_scl; // <-- last arg sets zoom range

//         // View transformation matrix: { position(x,y,z), direction(x,y,z), up_dim(x,y,z)}
//         let view = view_matrix(
//             &[    grid_ctr_x + cam_x,
//                 0.0 + cam_y,
//                 (grid_ctr_z * 0.4) + cam_z + -1.7],  // <-- second f32 sets z base
//             &[    0.0 - (cam_x / 5.0),
//                 0.0 - (cam_y / 5.0),
//                 0.5  * -grid_ctr_z],  // <-- first f32 sets distant focus point
//             &[0.0, 1.0, 0.0]
//         );

//         // Model transformation matrix:
//         // TODO: DEPRICATE
//         let grid_model = [
//             [1.0, 0.0, 0.0, 0.0],
//             [0.0, 1.0, 0.0, 0.0],
//             [0.0, 0.0, 1.0, 0.0],
//             [0.0, 0.0, 0.0, 1.0f32]
//         ];

//         // Light position:
//         let light_pos = [-1.0, 0.4, -0.9f32];

//         // // Model color (all three elements fluctuate):
//         // let global_color = [
//         //     (f32::abs(f32::cos(f_c / 3.0) * 0.99)) + 0.001,
//         //     (f32::abs(f32::sin(f_c / 2.0) * 0.99)) + 0.001,
//         //     (f32::abs(f32::cos(f_c / 1.0) * 0.99)) + 0.001,
//         // ];

//         // Model color (only blue fluctuates):
//         let global_color = [
//             0.0,
//             0.0,
//             // // 0% - 30% blue just for effect:
//             // (f32::abs(f32::cos(f_c) * 0.30)),
//             // 30% blue static:
//             0.3f32,
//         ];

//         // Uniforms:
//         let uniforms = uniform! {
//             model: grid_model,
//             view: view,
//             persp: persp,
//             u_light_pos: light_pos,
//             u_global_color: global_color,
//             grid_v_size: grid_dims.0,
//             grid_u_size: grid_dims.1,
//             // diffuse_tex: &diffuse_texture,
//             // normal_tex: &normal_map,
//         };

//         // // Draw Grid (without per-instance vertex buffer):
//         // target.draw((&self.vertices, glium::vertex::EmptyInstanceAttributes { len: grid_count }),
//         //     &self.indices, &self.program, &uniforms, &self.params).unwrap();

//         // Draw Grid (with per-instance vertex buffer):
//         target.draw((&self.vertices, hex_grid_buf.vertex_buf().per_instance().unwrap()),
//             &self.indices, &self.program, &uniforms, &self.params).unwrap();
//     }

//
