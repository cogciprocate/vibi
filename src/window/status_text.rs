// <<<<< THIS MODULE IS MARKED FOR DEPRICATION >>>>>

#![allow(dead_code, unused_variables)]
use glium_text::{self, TextSystem, FontTexture, TextDisplay};
use glium;
// use glium::{self, Surface};
// use glium::backend::{self, Facade};
// use glium::backend::glutin_backend::{ GlutinFacade };

use super::window_stats::WindowStats;
use interactive::CyStatus;

static TEXT_SCALE: f32 = 0.018;
static TEXT_COLOR: (f32, f32, f32, f32) = (0.99, 0.99, 0.99, 1.0);

// TODO: DEPRICATE
pub struct StatusText {
	text_system: TextSystem,
	font_texture: FontTexture,
	scale: f32,
	color: (f32, f32, f32, f32),
}

impl StatusText {
	pub fn new<F>(display: &F) -> StatusText 
			where F: glium::backend::Facade 
	{
		// Text system (experimental):
		let text_system = TextSystem::new(display);

		// Font:
		let font_size = 12;
		let font_texture = FontTexture::new(display, &include_bytes!(
				"/home/nick/projects/vibi/assets/fonts/NotoSans/NotoSans-Regular.ttf"
			)[..], font_size).unwrap();

		StatusText {
			text_system: text_system,
			font_texture: font_texture,
			scale: TEXT_SCALE,
			color: TEXT_COLOR,
		}
	}

	pub fn draw<S>(&self, target: &mut S, cycle_status: &CyStatus, window_stats: &WindowStats, 
				grid_dims: (u32, u32), area_name: &str)
			where S: glium::Surface
	{
		let (width, height) = target.get_dimensions();
		self.draw_text(&format!("Window: {} X {}", width, height), 6, 26, target);
		self.draw_text(&format!("Grid: {} X {}", grid_dims.0, grid_dims.1), 6, 56, target);
		self.draw_text(&format!("FPS: {:.1}", window_stats.fps()), 6, 86, target);
		self.draw_text(&format!("Current Cycle: {:.1}", cycle_status.cur_cycle), 6, 116, target);
		self.draw_text(&format!("Previous Cycles: {:.1}", cycle_status.ttl_cycles), 6, 146, target);
		self.draw_text(&format!("Current CPS: {:.1}", cycle_status.cur_cps()), 6, 176, target);
		self.draw_text(&format!("Average CPS: {:.1}", cycle_status.ttl_cps()), 6, 206, target);
		self.draw_text(&format!("Area: \"{}\"", area_name), 6, 236, target);
	}

	fn draw_text<S: glium::Surface>(&self, text: &str, x_off: u32, y_off: u32, target: &mut S) {
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

		glium_text::draw(&text_display, &self.text_system, target, text_xform, self.color);
	}
}



