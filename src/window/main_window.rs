// use std::iter;
use std::sync::mpsc::{Receiver, Sender};
use loop_cycles::{CyCtl, CySts};
use glium::{self, DisplayBuild, Surface};
// use glium::glutin::{ElementState};
use window::{util, C_ORANGE, /*INIT_GRID_SIZE,*/ MouseInputEventResult, KeyboardInputEventResult, 
	WindowStats, HexGrid, StatusText, UiPane, TextBox, HexButton};
use ganglion_buffer::GanglionBuffer;


// [FIXME]: Needs a rename. Anything containing 'Window' is misleading (UiPane is the window).
pub struct MainWindow {
	pub cycle_status: CySts,
	pub stats: WindowStats,
	pub close_pending: bool,
	pub grid_dims: (u32, u32),
	pub iters_pending: u32,
	pub control_tx: Sender<CyCtl>, 
	pub status_rx: Receiver<CySts>,
	// pub g_buf: GanglionBuffer,
}

impl MainWindow {
	pub fn open(control_tx: Sender<CyCtl>, status_rx: Receiver<CySts>) {		

		// Get initial cycle status so we know grid dims:
		let cy_sts_init = status_rx.recv().expect("Status receiver error.");		

		// Main window data struct:
		let mut window = MainWindow {
			cycle_status: CySts::new((0, 0)),
			stats: WindowStats::new(),
			close_pending: false,
			grid_dims: cy_sts_init.dims,
			iters_pending: 1,
			control_tx: control_tx,
			status_rx: status_rx,
			// g_buf: GanglionBuffer::new(grid_count, &display),
		};

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

		// Total hex tile count:
		let grid_count = (cy_sts_init.dims.0 * cy_sts_init.dims.1) as usize;

		// Ganglion buffer:
		let mut g_buf = GanglionBuffer::new(grid_count, &display);

		// Hex grid:
		let hex_grid = HexGrid::new(&display);

		// Status text UI element (fps & grid side):
		let status_text = StatusText::new(&display);


		// Primary user interface elements:
		let mut ui = UiPane::new(&display)
			// .element(HexButton::new([1.0, 1.0, 0.0], (-0.36, -0.07), 2.0, 
			// 		"Grid Size + 1", C_ORANGE)
			// 	.mouse_input_handler(Box::new(|_, _, window| { 
			// 			if window.grid_size < super::MAX_GRID_SIZE { window.grid_size += 1; }
			// 			MouseInputEventResult::None
			// 	}))
			// )			

			// .element(HexButton::new([1.0, 1.0, 0.0], (-0.36, -0.17), 2.0, 
			// 		"Grid Size - 1", C_ORANGE)
			// 	.mouse_input_handler(Box::new(|_, _, window| { 
			// 		if window.grid_size > 2 { window.grid_size -= 1; };
			// 		MouseInputEventResult::None
			// 	}))
			// )

			// .element(HexButton::new([1.0, 1.0, 0.0], (-0.095, -0.07), 0.22, 
			// 		"* 2", C_ORANGE)
			// 	.mouse_input_handler(Box::new(|_, _, window| { 
			// 			if window.grid_size < super::MAX_GRID_SIZE { window.grid_size *= 2; }
			// 			MouseInputEventResult::None
			// 	}))
			// )	

			// .element(HexButton::new([1.0, 1.0, 0.0], (-0.095, -0.17), 0.22, 
			// 		"/ 2", C_ORANGE)
			// 	.mouse_input_handler(Box::new(|_, _, window| { 
			// 		if window.grid_size >= 4 { window.grid_size /= 2; }
			// 		MouseInputEventResult::None
			// 	}))
			// )

			.element(TextBox::new([1.0, -1.0, 0.0], (-0.385, 0.500), 4.45, 
					"Iters:", C_ORANGE, "1", Box::new(|key_state, vk_code, kb_state, text_string, window| {
						util::key_into_string(key_state, vk_code, kb_state, text_string);

						if let Ok(i) = text_string.trim().replace("k","000").parse() {						
							window.iters_pending = i;
						}

						KeyboardInputEventResult::RequestRedraw
					})
				)
				.mouse_input_handler(Box::new(|_, _, _| MouseInputEventResult::RequestKeyboardFocus(true)))

			)

			.element(HexButton::new([1.0, -1.0, 0.0], (-0.57, 0.40), 1.8, 
					"Cycle", C_ORANGE)
				.mouse_input_handler(Box::new(|_, _, window| { 					
					window.control_tx.send(CyCtl::Iterate(window.iters_pending))
						.expect("Iterate button");
					MouseInputEventResult::None
				}))
			)

			.element(HexButton::new([1.0, -1.0, 0.0], (-0.20, 0.40), 1.8, 
					"Stop", C_ORANGE)
				.mouse_input_handler(Box::new(|_, _, window| { 					
					window.control_tx.send(CyCtl::Stop)
						.expect("Stop button");
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
			{mt}'Escape' or 'q' to quit.",
			mt = "    ");


		//////////////////////////////////////////////////////////////////////////
		///////////////////// Primary Event & Rendering Loop /////////////////////
		//////////////////////////////////////////////////////////////////////////
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

			// Refresh ganglion states:
			window.control_tx.send(CyCtl::Sample(g_buf.raw_states())).expect("Sample raw states");
			// g_buf.fill_rand();
			g_buf.refresh_v_buf();

			// Draw hex grid:
			hex_grid.draw(&mut target, window.grid_dims, window.stats.elapsed_ms(), g_buf.v_buf());

			// Draw FPS and grid side text:
			status_text.draw(&mut target, &window.stats, window.grid_dims);

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

		// Hide window when exiting.
		// [FIXME] TODO: Draw "Closing..." or something like that to the display instead.
		display.get_window().unwrap().hide();
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
