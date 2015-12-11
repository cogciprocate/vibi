// use std::thread;
use std::sync::mpsc::{ Receiver, Sender };
// use time::{ self, Timespec };
use find_folder::{ Search };
use conrod::{ Theme, Ui, };
use piston_window::{ PistonWindow, WindowSettings, Window, Glyphs, OpenGL, Size, Key, PressEvent, Button as KeyButton };

use piston_window::{ Flip, TextureSettings, Texture, image, clear };
// use opengl_graphics::glyph_cache::GlyphCache;
// use elmesque::{ self, Form, Renderer };


use loop_cycles::{ CyCtl, CySts };
use conrod_draw;
use win_stats::{ WinStats };


pub fn window(mut control_tx: Sender<CyCtl>, status_rx: Receiver<CySts>) {
	let win_size_init = Size { width: 1200, height: 800 };

	let window: PistonWindow = WindowSettings::new("Vibi", win_size_init)
		.opengl(OpenGL::V3_2)
		.exit_on_esc(true)
		// .vsync(true)
		.build().expect("Window build error");

	let (mut ui, rust_logo) = {
		let assets = Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
		let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
		let glyph_cache = Glyphs::new(&font_path, window.factory.borrow().clone()).unwrap();
		// let glyph_cache = GlyphCache::new(&font_path).unwrap();

		// Rust logo image asset:
		let rust_logo = assets.join("rust.png");
	    let rust_logo = Texture::from_path(
	            &mut *window.factory.borrow_mut(),
	            &rust_logo,
	            Flip::None,
	            &TextureSettings::new()
	        ).unwrap();

		(Ui::new(glyph_cache, Theme::default()), rust_logo)
	};


	

	// let bpos = (340.0, -350.0);
	let mut iters = "1".to_string();
	let mut cycle_status = CySts::new();
	let mut close_window = false;
	let mut stats = WinStats::new();

	// let stupid_labels = vec!["SHOT:30", "BABY-WOLF:45", "GIRLS:00", "KITTY:15"];
	// let mut stupid_labels_idx = 0;

	for e in window {
		// use conrod::{ color, Colorable, Labelable, Sizeable, Widget, Button, Positionable, TextBox,
		// 	Label, Ui, };
		// use piston_window::{ Glyphs, Size };
		// use interactive as iact;
		// use widgets::{ HexButton, /*HexGrid, DrinkingClock*/ };
		// const SHOW_FPS: bool = true;
		// widget_ids!(BUTTON_STOP, BUTTON_EXIT, BUTTON_CYCLE, TEXTBOX_ITERS, LABEL_CUR_CYCLE, LABEL_TTL_CYCLES,
		// 	LABEL_FPS, HEX_BUTTON, /*HEX_GRID, DRINKING_CLOCK*/);

		if let Some(KeyButton::Keyboard(key)) = e.press_args() {
            if key == Key::Q {
                println!("Pressed 'Q'.");
                close_window = true;
            }

            println!("Pressed keyboard key '{:?}'.", key);
        };

        ui.handle_event(&e.event.clone().unwrap());

        loop {
			match status_rx.try_recv() {
				Ok(cs) => {
					cycle_status = cs;
				},
				Err(_) => break,
			};
		}

		e.draw_2d(|c, g| {
			let win_size = e.size();

			// conrod::Background::new().rgb(0.2, 0.25, 0.4).set(&mut ui);			
			clear([0.2, 0.25, 0.4, 1.0], g);

			// let img_transform = math::transform_pos(, 
			image(&rust_logo, c.transform, g);
			

			///////////// NEW DRAW //////////////
			conrod_draw::draw_buttons(&mut ui, &mut cycle_status, &win_size, 
				&mut control_tx, &stats, &mut close_window, &mut iters);


			// let btn_size = Size { width: 130, height: 28 };

			// let b_r_corner = (
			// 	(win_size.width / 2) as f64 - (15 + (btn_size.width / 2)) as f64, 
			// 	(15 + (btn_size.height / 2)) as f64 - (win_size.height / 2) as f64
			// );

			// let t_r_corner = (
			// 	(win_size.width / 2) as f64 - (15 + (btn_size.width / 2)) as f64, 
			// 	(win_size.height / 2) as f64 - (15 + (btn_size.height / 2)) as f64
			// );

			// Button::new()
			// 	.color(color::dark_purple())
			// 	.label_font_size(12)
			// 	.label_color(color::grey())
			// 	.dimensions(btn_size.width as f64, btn_size.height as f64)
			// 	.xy(b_r_corner.0, b_r_corner.1)
			// 	.label("Exit")
			// 	.react(|| {
			// 		control_tx.send(CyCtl::Exit)
			// 			.expect("Exit button control tx");
			// 		close_window = true;
			// 	})
			// 	.set(BUTTON_EXIT, &mut ui);
			
			// Button::new()
			// 	.color(color::blue())
			// 	.label_font_size(12)
			// 	.label_color(color::grey())
			// 	.dimensions(btn_size.width as f64, btn_size.height as f64)
			// 	.relative_to(BUTTON_EXIT, [-(btn_size.width as f64 + 15.0), 
			// 		(btn_size.height as f64 + 15.0)])
			// 	.label("Cycle")
			// 	.react(|| {
			// 		match iters.trim().replace("k","000").parse() {						
			// 			Ok(i) => control_tx.send(CyCtl::Iterate(i))
			// 				.expect("Iterate button control tx"),
			// 			Err(_) => (),
			// 		}
			// 	})
			// 	.set(BUTTON_CYCLE, &mut ui);

			// Button::new()
			// 	.color(color::blue())
			// 	.label_font_size(12)
			// 	.label_color(color::grey())
			// 	.dimensions(btn_size.width as f64, btn_size.height as f64)
			// 	.right_from(BUTTON_CYCLE, 15.0)
			// 	.label("Stop")
			// 	.react(|| {
			// 		control_tx.send(CyCtl::Stop)
			// 			.expect("Stop button control tx");
			// 	})
			// 	.set(BUTTON_STOP, &mut ui);

			// TextBox::new(&mut iters)
			// 	.font_size(12)
			// 	.dimensions(btn_size.width as f64, btn_size.height as f64)
			// 	.up_from(BUTTON_CYCLE, 15.0)
			// 	.color(color::blue().invert())
			// 	.react(|_string: &mut String| {})
			// 	.set(TEXTBOX_ITERS, &mut ui);

			// Label::new(&format!("Current Cycle: {}/{}", cycle_status.cur_cycle, 
			// 			iact::parse_iters(&iters).unwrap_or(0)))
			// 	.up_from(TEXTBOX_ITERS, btn_size.height as f64 + 10.0)
			// 	.font_size(12)
			// 	.color(color::orange())
			// 	.set(LABEL_CUR_CYCLE, &mut ui);

			// Label::new(&format!("Total Cycles: {} (@ {:.2}/s)", cycle_status.ttl_cycles,
			// 		cycle_status.ttl_cps()))
			// 	.down_from(LABEL_CUR_CYCLE, 10.0)
			// 	.font_size(12)
			// 	.color(color::orange())
			// 	.set(LABEL_TTL_CYCLES, &mut ui);

			// if SHOW_FPS {
			// 	Label::new(&format!("FPS: {:.2}/s", stats.fps()))
			// 		.left_from(BUTTON_EXIT, 300.0)
			// 		.font_size(12)
			// 		.color(color::orange())
			// 		.set(LABEL_FPS, &mut ui);
			// }

			// HexButton::new()
			// 	.color(color::blue())
			// 	.dimensions(128.0, 128.0)
			// 	.xy(t_r_corner.0 - 22.0, t_r_corner.1 - 64.0)
			// 	.orient(0.0)
			// 	.label_color(color::grey())
			// 	.label_font_size(14)
			// 	.label("Lucky Button")
			// 	.react(|| {
			// 		println!("You clicked the lucky button! Your luck score has increased by \
			// 			{}!", cycle_status.ttl_cycles + cycle_status.cur_cycle);
			// 	})
			// 	.set(HEX_BUTTON, &mut ui);


			ui.draw_if_changed(c, g);
		});

		if close_window { break; }
		stats.incr();
	}
}


// struct WinStats {
// 	pub event_count: usize,
// 	pub start_time: Timespec,
// }

// #[allow(dead_code)]
// impl WinStats {
// 	pub fn new() -> WinStats {
// 		// panic!("Replaced by new version");
// 		WinStats {
// 			event_count: 0usize,
// 			start_time: time::get_time(),
// 		}
// 	}

// 	fn fps(&self) -> f32 {
// 		(self.event_count as f32 / (time::get_time() - self.start_time)
// 			.num_milliseconds() as f32) * 1000.0
// 	}

// 	fn elapsed_secs(&self) -> f32 {
// 		(time::get_time() - self.start_time).num_seconds() as f32
// 	}

// 	/// Returns microseconds elapsed since the window was created (mu = μ).
// 	fn elapsed_mus(&self) -> f64 {
// 		(time::get_time() - self.start_time).num_microseconds().unwrap() as f64
// 	}

// 	/// Returns milliseconds elapsed since the window was created.
// 	fn elapsed_ms(&self) -> f64 {
// 		(time::get_time() - self.start_time).num_milliseconds() as f64
// 	}
// }







			// HexGrid::new()
			// 	.color(color::blue())
			// 	.dimensions(1200.0, 800.0)
			// 	.xy(0.0, 0.0)
			// 	.depth(1.0)
			// 	// .orient(0.0)
			// 	.label_color(conrod::color::white())
			// 	.label_font_size(18)
			// 	.whatever_number(stats.elapsed_ms() / 8000.0)
			// 	.label(stupid_labels[stupid_labels_idx])
			// 	.react(|| {
			// 		stupid_labels_idx = if stupid_labels_idx >= (stupid_labels.len() - 1) {
			// 			0 
			// 		} else {
			// 			stupid_labels_idx + 1
			// 		}
			// 	})
			// 	.set(HEX_GRID, &mut ui);

			// DrinkingClock::new()
			// 	.color(color::blue())
			// 	.dimensions(1200.0, 800.0)
			// 	.xy(0.0, 0.0)
			// 	.depth(1.0)
			// 	// .orient(0.0)
			// 	.label_color(conrod::color::white())
			// 	.label_font_size(18)
			// 	.whatever_number(stats.elapsed_ms() / 8000.0)
			// 	.label(stupid_labels[stupid_labels_idx])
			// 	.react(|| {
			// 		stupid_labels_idx = if stupid_labels_idx >= (stupid_labels.len() - 1) {
			// 			0 
			// 		} else {
			// 			stupid_labels_idx + 1
			// 		}
			// 	})
			// 	.set(DRINKING_CLOCK, &mut ui);


