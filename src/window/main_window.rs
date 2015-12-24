use std::sync::mpsc::{Receiver, Sender};
use loop_cycles::{CyCtl, CySts};
use glium::{self, DisplayBuild, Surface};
use glium::glutin::{ElementState};
use window::{C_ORANGE, INIT_GRID_SIZE, util, MouseInputEventResult, KeyboardInputEventResult, 
	WindowStats, HexGrid, StatusText, UiPane, TextBox, HexButton};
// use glium::window::{Window};


pub struct MainWindow {
	pub cycle_status: CySts,
	pub stats: WindowStats,
	pub close_pending: bool,
	pub grid_size: u32,
	pub iters_pending: u32,
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
			iters_pending: 1,
			control_tx: control_tx,
			status_rx: status_rx,
		};

		// Create our window:
		let display: glium::backend::glutin_backend::GlutinFacade = glium::glutin::WindowBuilder::new()
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
			
			.element(HexButton::new([1.0, 1.0, 0.0], (-0.25, -0.07), 2.5, 
					"+ Grid Size", C_ORANGE)
				.mouse_input_handler(Box::new(|_, _, window| { 
						window.grid_size += 1;
						MouseInputEventResult::None
				}))
			)

			.element(HexButton::new([1.0, 1.0, 0.0], (-0.25, -0.17), 2.5, 
					"- Grid Size", C_ORANGE)
				.mouse_input_handler(Box::new(|_, _, window| { 
					window.grid_size -= 1;
					MouseInputEventResult::None
				}))
			)

			.element(TextBox::new([1.0, -1.0, 0.0], (-0.39, 0.50), 4.5, 
					"Iters:", C_ORANGE, "1", Box::new(|key_state, vk_code, kb_state, text_string, window| {
						if let ElementState::Pressed = key_state {
							use glium::glutin::VirtualKeyCode::*;
							match vk_code {
								Some(Back) => {
									text_string.pop();
									// return KeyboardInputEventResult::PopTextString;
								},

								_ => {
									if let Some(mut c) = util::map_vkc(vk_code) {					
										// println!("    VirtualKeyCode: {:?} => {:?}", vk_code, c);
										if kb_state.shift { c = c.to_uppercase().next().unwrap_or(c); }
										text_string.push(c);

										if let Ok(i) = text_string.trim().replace("k","000").parse() {						
											window.iters_pending = i;
										}
										// return KeyboardInputEventResult::PushTextString(c);
									}
								},
							}
						}

						KeyboardInputEventResult::None
					})
				)
				.mouse_input_handler(Box::new(|_, _, _| MouseInputEventResult::RequestKeyboardFocus(true)))

			)

			.element(HexButton::new([1.0, -1.0, 0.0], (-0.20, 0.40), 1.8, 
					"Cycle", C_ORANGE)
				.mouse_input_handler(Box::new(|_, _, window| { 					
					window.control_tx.send(CyCtl::Iterate(window.iters_pending))
						.expect("Iterate button control tx");
					MouseInputEventResult::None
				}))
			)

			.element(HexButton::new([1.0, -1.0, 0.0], (-0.20, 0.07), 1.8, 
					"Exit", C_ORANGE)
				.mouse_input_handler(Box::new(|_, _, window| { 
					window.close_pending = true;
					MouseInputEventResult::None
				}))
			)			

			.init();		


		// Print some stuff:
		println!("\n==================== Vibi Keyboard Bindings ===================\n\
			{mt}The following keys must be used with 'ctrl':\n\
			{mt}'Escape' or 'q' to quit.\n\
			{mt}'Up Arrow' to double or 'Down Arrow' to halve grid size.\n\
			{mt}'Right Arrow' to increase or 'Left Arrow' to decrease grid size by one.\n",
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
