// <<<<< THIS MODULE IS MARKED FOR DEPRICATION >>>>>
// TODO: Convert this into a UiElement or something.

#![allow(dead_code, unused_variables)]
use glium_text_rusttype::{self, TextSystem, FontTexture, TextDisplay};
use glium;
use glium::backend::glutin::Display;
// use glium::backend::Facade;
// use glium::{self, Surface};
// use glium::backend::{self, Facade};
// use glium::backend::glutin_backend::{ GlutinFacade };

use window::WindowStats;
use bismit::flywheel::Status;
use bismit::map::SliceTractMap;

const TEXT_SCALE: f32 = 0.036;
const TEXT_COLOR: (f32, f32, f32, f32) = (0.99, 0.99, 0.99, 1.0);

// TODO: DEPRICATE
pub struct StatusText {
    text_system: TextSystem,
    font_texture: FontTexture,
    scale: f32,
    color: (f32, f32, f32, f32),
}

impl StatusText {
    pub fn new(display: &Display) -> StatusText {
        // Text system (experimental):
        let text_system = TextSystem::new(display);

        // Font:
        let font_size = 36;
        let font_texture = FontTexture::new(display, &include_bytes!(
                "assets/fonts/NotoSans/NotoSans-Regular.ttf"
            )[..], font_size, FontTexture::ascii_character_list()).unwrap();

        StatusText {
            text_system: text_system,
            font_texture: font_texture,
            scale: TEXT_SCALE,
            color: TEXT_COLOR,
        }
    }

    fn draw_line<S: glium::Surface>(&self, text: &str, x_off: u32, y_off: u32, target: &mut S) {
        let (width, height) = target.get_dimensions();
        let text_x_scl = self.scale / (width as f32 / 1000.0);
        let text_y_scl = text_x_scl * (width as f32) / (height as f32);

        let text_xform = [
            [text_x_scl, 0.0, 0.0, 0.0,],
            [0.0, text_y_scl, 0.0, 0.0,],
            [0.0, 0.0, 1.0, 0.0,],
            [-1.0 + (x_off as f32 / width as f32), 1.0 - (y_off as f32 / height as f32), 0.0, 1.0f32,],
        ];

        let text_display = TextDisplay::new(&self.text_system, &self.font_texture, text);

        glium_text_rusttype::draw(&text_display, &self.text_system, target, text_xform, self.color)
            .unwrap();
    }

    pub fn draw<S: glium::Surface>(&self, target: &mut S, cycle_status: &Status,
                window_stats: &WindowStats, grid_dims: (u32, u32), area_name: &str, cam_dst: f32,
                top_right: [f32; 4], cam_pos_raw: [f32; 3], tract_map: &SliceTractMap)
    {
        let (width, height) = target.get_dimensions();
        self.draw_line(&format!("Camera Pos: ({}, {}, {})", cam_pos_raw[0],
            cam_pos_raw[1], cam_pos_raw[2]), 6, 26, target);
        self.draw_line(&format!("View Distance: {:.0}%", cam_dst * 100.0), 6, 56, target);
        self.draw_line(&format!("Window: {} X {}", width, height), 6, 86, target);
        self.draw_line(&format!("FPS: {:.1}", window_stats.fps()), 6, 116, target);
        self.draw_line(&format!("Current Cycle: {:.1}", cycle_status.cur_cycle()), 6, 146, target);
        self.draw_line(&format!("Current CPS: {:.1}", cycle_status.cur_cps()), 6, 176, target);
        self.draw_line(&format!("Total Cycles: {:.1}", cycle_status.ttl_cycles()), 6, 206, target);
        self.draw_line(&format!("Total CPS: {:.1}", cycle_status.ttl_cps()), 6, 236, target);
        self.draw_line(&format!("Area Name: \"{}\"", area_name), 6, 266, target);
        self.draw_line(&format!("Area Size: {} X {}", grid_dims.0, grid_dims.1), 6, 296, target);
        self.draw_line(&format!("Layers: {:?}", tract_map.tags_reversed()), 6, 326, target);
        // self.draw_line(&format!("Top Right Corner: ({}, {})",
        //     top_right[0], top_right[1]), 6, 296, target);


    }
}



