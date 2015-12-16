pub use self::window_stats::WindowStats;
pub use self::status_text::StatusText;
pub use self::hex_grid::HexGrid;
pub use self::ui::Ui;
pub use self::ui_element::UiElement;
pub use self::ui_vertex::UiVertex;
pub use self::mouse_state::MouseState;

use std::sync::mpsc::{Receiver, Sender};
use loop_cycles::{CyCtl, CySts};

mod window_stats;
mod status_text;
mod hex_grid;
mod ui;
mod ui_element;
mod ui_vertex;
mod mouse_state;
// mod conrod;
// mod window_grid;
// pub mod window_main;

pub const C_PINK: [f32; 3] = [0.990, 0.490, 0.700];
pub const C_ORANGE: [f32; 3] = [0.960, 0.400, 0.0];

pub const INIT_GRID_SIZE: u32 = 64;
pub const MAX_GRID_SIZE: u32 = 8192;

pub struct Window {
	cycle_status: CySts,
	stats: WindowStats,
	close_pending: bool,
	grid_size: u32,
}

impl Window {
	pub fn open(control_tx: Sender<CyCtl>, status_rx: Receiver<CySts>) {
		// use std::f32;
		// use std::thread;
		// use time::{self, Timespec, Duration};
		// use std::io::{Cursor};		
		use glium::{self, DisplayBuild, Surface};
		// use glium::backend::glutin_backend::{GlutinFacade};
		// use image;
		// use find_folder::{Search};

		// Create our window:
		let display = glium::glutin::WindowBuilder::new()
			.with_depth_buffer(24)
			.with_dimensions(1400, 800)
			.with_title("Vibi".to_string())
			.with_multisampling(8)
			// .with_gl_robustness(glium::glutin::Robustness::NoError) // <-- Disabled for development
			.with_vsync()
			// .with_transparency(true)
			// .with_fullscreen(glium::glutin::get_primary_monitor())
			.build_glium().unwrap();

		// Hex grid:
		let hex_grid = HexGrid::new(&display);

		// Status text UI element (fps & grid side):
		let status_text = StatusText::new(&display);

		// Primary user interface:
		let mut ui = Ui::new(&display)
			// .element(UiElement::hex_button([anchor: x, y, z], (offset: x, y), scale, extra_width, text))
			// .element(UiElement::hex_button([1.0, 1.0, 0.0], (-0.06, -0.06), 0.06, 0.0, "yo")
			// .element(UiElement::hex_button([-1.0, -1.0, 0.0], (0.06, 0.06), 0.06, 0.0, "yo")
			.element(UiElement::hex_button([1.0, -1.0, 0.0], (-0.52, 0.06), 0.06, 2.0, 
				"Settings".to_string(), C_ORANGE))
			.element(UiElement::hex_button([1.0, -1.0, 0.0], (-0.18, 0.06), 0.06, 2.0, 
				"Exit".to_string(), C_ORANGE))
			.init();

		
		let mut window = Window {
			cycle_status: CySts::new(),
			stats: WindowStats::new(),
			close_pending: false,
			grid_size: INIT_GRID_SIZE,
		};


		// Print some stuff:
		println!("\n==================== Vibi Experimental Window ====================\n\
			Press 'Escape' or 'Q' to quit.\n\
			Press 'Up Arrow' to double or 'Down Arrow' to halve grid size.\n\
			Press 'Right Arrow' to increase or 'Left Arrow' to decrease grid size by one.");

		// Event/Rendering loop:
		loop {
			ui.set_input_stale();
			// Check cycle status:
			loop {
				match status_rx.try_recv() {
					Ok(cs) => {
						window.cycle_status = cs;
					},
					Err(_) => break,
				};
			}

			// Check input events:
			for ev in display.poll_events() {
				ui.handle_event(ev, &mut window);
			}

			// Create draw target and clear color and depth:
			let mut target = display.draw();
			target.clear_color_and_depth((0.030, 0.050, 0.080, 1.0), 1.0);

			// Draw hex grid:
			hex_grid.draw(&mut target, window.grid_size, window.stats.elapsed_ms());

			// Draw FPS and grid side text:
			status_text.draw(&mut target, &window.stats, window.grid_size);

			// Draw UI:
			ui.draw(&mut target);

			// Swap buffers:
			target.finish().unwrap();

			// Increment our counters:
			window.stats.incr();

			// Clean up and exit if necessary:
			if window.close_pending {
				control_tx.send(CyCtl::Exit).expect("Exit button control tx");
				break;
			}


			/////////// DEBUG STUFF ////////////
				// if !ui.input_is_stale() {
				// 	println!("##### Mouse position: {:?}", ui.mouse_state().position());
				// }



			////////////////////////////////////
		}
	}
}
