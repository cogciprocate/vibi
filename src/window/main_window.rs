use std::sync::mpsc::{Receiver, Sender};
use loop_cycles::{CyCtl, CySts};
use glium::{self, DisplayBuild, Surface};

use super::{C_ORANGE, INIT_GRID_SIZE, WindowStats, HexGrid, StatusText,
	UiPane, TextBox, HexButton};


pub struct MainWindow {
	pub cycle_status: CySts,
	pub stats: WindowStats,
	pub close_pending: bool,
	pub grid_size: u32,
	pub control_tx: Sender<CyCtl>, 
	pub status_rx: Receiver<CySts>,
}

impl MainWindow {
	pub fn open(control_tx: Sender<CyCtl>, status_rx: Receiver<CySts>) {	
		let mut window = MainWindow {
			cycle_status: CySts::new(),
			stats: WindowStats::new(),
			close_pending: false,
			grid_size: INIT_GRID_SIZE,
			control_tx: control_tx,
			status_rx: status_rx,
		};

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


		// Primary user interface elements:
		let mut ui = UiPane::new(&display)
			// KEY: ::hex_button([anchor: x, y, z], (offset: x, y), extra_width, text, color, click handler))
			
			.element(HexButton::new([1.0, 1.0, 0.0], (-0.22, -0.07), 2.5, 
					"+ Grid Size".to_string(), C_ORANGE)
				.click_action(Box::new(|window| { 
						// println!("Shrinking Grid..."); 
						window.grid_size += 1;
				}))
			)

			// .element(HexButton::new([1.0, 1.0, 0.0], (-0.32, -0.12), 2.5, 
			// 		"Ahem".to_string(), [0.3, 0.3, 0.3])
			// 	.click_action(Box::new(|_| { 
			// 			// println!("Shrinking Grid..."); 
			// 			// window.grid_size += 1;
			// 	}))
			// )

			.element(HexButton::new([1.0, 1.0, 0.0], (-0.22, -0.17), 2.5, 
					"- Grid Size".to_string(), C_ORANGE)
				.click_action(Box::new(|window| { 
					// println!("Shrinking Grid..."); 
					window.grid_size -= 1;
				}))
			)

			.element(TextBox::new([-1.0, -1.0, 0.0], (0.34, 0.50), 4.5, 
					"Iters:".to_string(), C_ORANGE,))

			.element(HexButton::new([1.0, -1.0, 0.0], (-0.18, 0.07), 1.8, 
					"Exit".to_string(), C_ORANGE)
				.click_action(Box::new(|window| { 
					// println!("Exit clicked."); 
					window.close_pending = true;
				}))
			)

			.init();		


		// Print some stuff:
		println!("\n==================== Vibi Keyboard Bindings ===================\n\
			{mt}Press 'Escape' or 'q' to quit.\n\
			{mt}Press 'Up Arrow' to double or 'Down Arrow' to halve grid size.\n\
			{mt}Press 'Right Arrow' to increase or 'Left Arrow' to decrease grid size by one.\n",
			mt = "    ");

		// Event/Rendering loop:
		loop {
			ui.set_input_stale();

			// Check cycle status:
			window.try_recv();			

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
				window.control_tx.send(CyCtl::Exit).expect("Exit button control tx");
				break;
			}


			/////////// DEBUG STUFF ////////////
				// if !ui.input_is_stale() {
				// 	println!("##### Mouse position: {:?}", ui.mouse_state().position());
				// }



			////////////////////////////////////
		}
	}

	fn try_recv(&mut self) {
		loop {
			match self.status_rx.try_recv() {
				Ok(cs) => {
					self.cycle_status = cs;
				},
				Err(_) => break,
			};
		}
	}
}
