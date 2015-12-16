#![allow(dead_code)]
use std::sync::mpsc::{ /*Receiver,*/ Sender };

use find_folder::{ Search };
use conrod::{ color, Colorable, Labelable, Sizeable, Widget, Button, Positionable, TextBox,
	Label, Ui, Theme };
use piston_window::{ PistonWindow, Glyphs, Size };
// use piston_window::{ Flip, TextureSettings, Texture, /*image, clear*/ };


use interactive as iact;
use loop_cycles::{ CyCtl, CySts };
use widgets::{ HexButton, /*HexGrid, DrinkingClock*/ };
use window_stats::{ WindowStats };

const SHOW_FPS: bool = true;

widget_ids!(BUTTON_STOP, BUTTON_EXIT, BUTTON_CYCLE, TEXTBOX_ITERS, LABEL_CUR_CYCLE, LABEL_TTL_CYCLES,
	LABEL_FPS, HEX_BUTTON, /*HEX_GRID, DRINKING_CLOCK*/);


// let (mut ui, rust_logo) = 

pub fn create_ui(window: PistonWindow) -> Ui<Glyphs> {
	let assets = Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
	let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
	let glyph_cache = Glyphs::new(&font_path, window.factory.borrow().clone()).unwrap();

	// Rust logo image asset:
	// let rust_logo = assets.join("rust.png");
 //    let rust_logo = Texture::from_path(
 //            &mut *window.factory.borrow_mut(),
 //            &rust_logo,
 //            Flip::None,
 //            &TextureSettings::new()
 //        ).unwrap();

	Ui::new(glyph_cache, Theme::default())
}


pub fn draw_buttons(ui: &mut Ui<Glyphs>, cycle_status: &mut CySts, win_size: &Size, 
			control_tx: &mut Sender<CyCtl>, stats: &WindowStats, close_window: &mut bool,
			mut iters_text: &mut String
		) 
{
	let btn_size = Size { width: 130, height: 28 };

	let b_r_corner = (
		(win_size.width / 2) as f64 - (15 + (btn_size.width / 2)) as f64, 
		(15 + (btn_size.height / 2)) as f64 - (win_size.height / 2) as f64
	);

	let t_r_corner = (
		(win_size.width / 2) as f64 - (15 + (btn_size.width / 2)) as f64, 
		(win_size.height / 2) as f64 - (15 + (btn_size.height / 2)) as f64
	);

	Button::new()
		.color(color::dark_purple())
		.label_font_size(12)
		.label_color(color::grey())
		.dimensions(btn_size.width as f64, btn_size.height as f64)
		.xy(b_r_corner.0, b_r_corner.1)
		.label("Exit")
		.react(|| {
			control_tx.send(CyCtl::Exit)
				.expect("Exit button control tx");
			*close_window = true;
		})
		.set(BUTTON_EXIT, ui);

	Button::new()
		.color(color::blue())
		.label_font_size(12)
		.label_color(color::grey())
		.dimensions(btn_size.width as f64, btn_size.height as f64)
		.relative_to(BUTTON_EXIT, [-(btn_size.width as f64 + 15.0), 
			(btn_size.height as f64 + 15.0)])
		.label("Cycle")
		.react(|| {
			match iters_text.trim().replace("k","000").parse() {						
				Ok(i) => control_tx.send(CyCtl::Iterate(i))
					.expect("Iterate button control tx"),
				Err(_) => (),
			}
		})
		.set(BUTTON_CYCLE, ui);

	Button::new()
		.color(color::blue())
		.label_font_size(12)
		.label_color(color::grey())
		.dimensions(btn_size.width as f64, btn_size.height as f64)
		.right_from(BUTTON_CYCLE, 15.0)
		.label("Stop")
		.react(|| {
			control_tx.send(CyCtl::Stop)
				.expect("Stop button control tx");
		})
		.set(BUTTON_STOP, ui);

	TextBox::new(&mut iters_text)
		.font_size(12)
		.dimensions(btn_size.width as f64, btn_size.height as f64)
		.up_from(BUTTON_CYCLE, 15.0)
		.color(color::blue().invert())
		.react(|_string: &mut String| {})
		.set(TEXTBOX_ITERS, ui);

	Label::new(&format!("Current Cycle: {}/{}", cycle_status.cur_cycle, 
				iact::parse_iters(&iters_text).unwrap_or(0)))
		.up_from(TEXTBOX_ITERS, btn_size.height as f64 + 10.0)
		.font_size(12)
		.color(color::orange())
		.set(LABEL_CUR_CYCLE, ui);

	Label::new(&format!("Total Cycles: {} (@ {:.2}/s)", cycle_status.ttl_cycles,
			cycle_status.ttl_cps()))
		.down_from(LABEL_CUR_CYCLE, 10.0)
		.font_size(12)
		.color(color::orange())
		.set(LABEL_TTL_CYCLES, ui);

	if SHOW_FPS {
		Label::new(&format!("FPS: {:.2}/s", stats.fps()))
			.left_from(BUTTON_EXIT, 300.0)
			.font_size(12)
			.color(color::orange())
			.set(LABEL_FPS, ui);
	}

	HexButton::new()
		.color(color::blue())
		.dimensions(128.0, 128.0)
		.xy(t_r_corner.0 - 22.0, t_r_corner.1 - 64.0)
		.orient(0.0)
		.label_color(color::grey())
		.label_font_size(14)
		.label("Lucky Button")
		.react(|| {
			println!("You clicked the lucky button! Your luck score has increased by \
				{}!", cycle_status.ttl_cycles + cycle_status.cur_cycle);
		})
		.set(HEX_BUTTON, ui);

}
