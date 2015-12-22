// <<<<< THIS MODULE IS MARKED FOR DEPRICATION >>>>>

#![allow(dead_code, unused_variables)]
use glium_text::{self, TextSystem, FontTexture, TextDisplay};
use glium;
// use glium::{self, Surface};
// use glium::backend::{self, Facade};
// use glium::backend::glutin_backend::{ GlutinFacade };

use super::window_stats::{WindowStats};


// TODO: DEPRICATE
pub struct StatusText {
	text_system: TextSystem,
	font_texture: FontTexture,
}

impl StatusText {
	pub fn new<F>(display: &F) -> StatusText 
			where F: /*glium::Surface +*/ glium::backend::Facade 
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
		}
	}

	pub fn draw<F>(&self, target: &mut F, stats: &WindowStats, grid_side: u32) 
			where F: glium::Surface /*+ glium::backend::Facade*/
	{
		// let text_model_matrix = [
		// 	[2.0 / text_width, 0.0, 0.0, 0.0,],
		// 	[0.0, 2.0 * (width as f32) / (height as f32) / text_width, 0.0, 0.0,],
		// 	[0.0, 0.0, 1.0, 0.0,],
		// 	[-1.0, -1.0, 0.0, 1.0f32,],
		// ];

		let (width, height) = target.get_dimensions();
		
		let text_scl = 0.018; // / ((width * height) as f32 / 1000000.0);
		// let text_x_scl = text_scl * 2.0 / text_width;
		// let text_y_scl = text_scl * 2.0 * (width as f32) / (height as f32) / text_width;

		let text_x_scl = text_scl / (width as f32 / 1000.0);
		let text_y_scl = text_x_scl * (width as f32) / (height as f32);


		////////////////// Window //////////////////
		let gs_text_matrix = [
			[text_x_scl, 0.0, 0.0, 0.0,],
			[0.0, text_y_scl, 0.0, 0.0,],
			[0.0, 0.0, 1.0, 0.0,],
			[-1.0 + (6 as f32 / width as f32), 1.0 - (26 as f32 / height as f32), 0.0, 1.0f32,],
		];

		let gs_text = TextDisplay::new(&self.text_system, &self.font_texture, 
			&format!("Window: {} X {}", width, height));

		glium_text::draw(&gs_text, &self.text_system, target, gs_text_matrix, 
			(0.99, 0.99, 0.99, 1.0));


		////////////////// Grid //////////////////
		let gs_text_matrix = [
			[text_x_scl, 0.0, 0.0, 0.0,],
			[0.0, text_y_scl, 0.0, 0.0,],
			[0.0, 0.0, 1.0, 0.0,],
			[-1.0 + (6 as f32 / width as f32), 1.0 - (56 as f32 / height as f32), 0.0, 1.0f32,],
		];

		let gs_text = TextDisplay::new(&self.text_system, &self.font_texture, 
			&format!("Grid: {gs} X {gs}", gs = grid_side));

		glium_text::draw(&gs_text, &self.text_system, target, gs_text_matrix, 
			(0.99, 0.99, 0.99, 1.0));


		///////////////////// FPS ////////////////////
		let fps_text_xform = [
			[text_x_scl, 0.0, 0.0, 0.0,],
			[0.0, text_y_scl, 0.0, 0.0,],
			[0.0, 0.0, 1.0, 0.0,],
			[-1.0 + (6 as f32 / width as f32), 1.0 - (86 as f32 / height as f32), 0.0, 1.0f32,],
		];

		let fps_text = TextDisplay::new(&self.text_system, &self.font_texture, 
			&format!("FPS: {:.1}", stats.fps()));

		glium_text::draw(&fps_text, &self.text_system, target, fps_text_xform, 
			(0.99, 0.99, 0.99, 1.0));

	}
}
