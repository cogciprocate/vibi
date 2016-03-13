use std::sync::mpsc::{Receiver, Sender};
use time::{self, Timespec, Duration};
use glium::{self, DisplayBuild, Surface};
use cycle::{CyCtl, CyRes, Status, AreaInfo};
use window::{HexGrid, StatusText};
use enamel::{self, Pane, EventRemainder, UiRequest, TextBox, HexButton, ElementState, 
    MouseButton, MouseScrollDelta, MouseState};
// use super::HexGridBuffer;

#[derive(Clone, Debug)]
pub enum HexGridCtl {
    SlcRangeDefault,
    SlcRangeFull,
}


#[derive(Clone, Debug)]
pub enum WindowCtl {
    None,
    Closed,
    MouseMoved((i32, i32)),
    MouseWheel(MouseScrollDelta),
    HexGrid(HexGridCtl),
    SetCyIters(u32),
    CyIterate,
    CyCtl(CyCtl),
}

impl EventRemainder for WindowCtl {
    fn closed() -> WindowCtl {
        WindowCtl::Closed
    }

    fn mouse_moved(pos: (i32, i32)) -> Self {
        WindowCtl::MouseMoved(pos)
    }

    fn mouse_wheel(delta: MouseScrollDelta) -> Self {
        WindowCtl::MouseWheel(delta)
    }
}

impl Default for WindowCtl {
    fn default() -> WindowCtl {
        WindowCtl::None
    }
}


pub struct WindowStats {
    pub frame_count: usize,
    pub start_time: Timespec,
    prev_event: Timespec,
    cur_fps: f32,
}

#[allow(dead_code)]
impl WindowStats {
    pub fn new() -> WindowStats {
        WindowStats {
            frame_count: 0usize,
            start_time: time::get_time(),
            prev_event: time::get_time(),
            cur_fps: 0.0,
        }
    }

    pub fn fps(&self) -> f32 {
        // (self.event_count as f32 / (time::get_time() - self.start_time)
        //     .num_milliseconds() as f32) * 1000.0
        self.cur_fps
    }

    pub fn elapsed_secs(&self) -> f32 {
        (time::get_time() - self.start_time).num_seconds() as f32
    }

    /// Returns microseconds elapsed since the window was created (mu = μ).
    pub fn elapsed_mus(&self) -> f64 {
        (time::get_time() - self.start_time).num_microseconds().unwrap() as f64
    }

    /// Returns milliseconds elapsed since the window was created.
    pub fn elapsed_ms(&self) -> f64 {
        (time::get_time() - self.start_time).num_milliseconds() as f64
    }

    /// Increment the frame counter by one and calculate fps for previous frame.
    pub fn incr(&mut self) {
        let now = time::get_time();

        let prev_frame_dur = now - self.prev_event;
        self.cur_fps = Duration::seconds(1).num_microseconds().unwrap() as f32
            / prev_frame_dur.num_microseconds().unwrap() as f32;

        self.frame_count += 1;
        self.prev_event = now;
    }
}



// [FIXME]: Needs a rename. Anything containing 'Window' is misleading (Pane is the window).
pub struct Window<'d> {
    pub cycle_status: Status,
    pub area_info: AreaInfo,
    pub stats: WindowStats,
    pub close_pending: bool,
    pub grid_dims: (u32, u32),
    pub iters_pending: u32,
    pub control_tx: Sender<CyCtl>, 
    pub result_rx: Receiver<CyRes>,
    pub hex_grid: HexGrid<'d>,
}

impl<'d> Window<'d> {
    pub fn open(control_tx: Sender<CyCtl>, result_rx: Receiver<CyRes>) {
        // Get initial area info:
        control_tx.send(CyCtl::RequestCurrentAreaInfo).expect("Error requesting current area name.");
        let area_info = match result_rx.recv().expect("Current area name reception error.") {
            CyRes::AreaInfo(box info) => info,
            _ => panic!("Invalid area name response."),
        };        

        let display: glium::backend::glutin_backend::GlutinFacade = glium::glutin::WindowBuilder::new()
            .with_depth_buffer(24)
            .with_dimensions(1400, 800)
            .with_title("Vibi".to_string())
            .with_multisampling(8)
            // Disabled for development ->> .with_gl_robustness(glium::glutin::Robustness::NoError)
            .with_vsync()
            // .with_transparency(true)
            // .with_fullscreen(glium::glutin::get_primary_monitor())
            .build_glium().unwrap();

        // Hex grid:
        let hex_grid = HexGrid::new(&display, area_info.clone());

        // Status text UI element (fps & grid side):
        let status_text = StatusText::new(&display);

        // Primary user interface elements:
        let mut ui = Pane::new(&display)
            .element(HexButton::new([1.0, -1.0, 0.0], (-0.57, 0.45), 1.8, 
                    "View Output", enamel::ui::C_ORANGE)
                .mouse_event_handler(Box::new(|_, _| {
                    // window.control_tx.send(CyCtl::Iterate(window.iters_pending))
                    //     .expect("View All Button button");
                    // window.hex_grid.buffer.use_default_slc_range();
                    // EventRemainder::None
                    (UiRequest::None, WindowCtl::HexGrid(HexGridCtl::SlcRangeDefault))
                }))
            )

            .element(HexButton::new([1.0, -1.0, 0.0], (-0.20, 0.45), 1.8, 
                    "View All", enamel::ui::C_ORANGE)
                .mouse_event_handler(Box::new(|_, _| {
                    // window.control_tx.send(CyCtl::Iterate(window.iters_pending))
                    //     .expect("View All Button button");
                    // window.hex_grid.buffer.use_full_slc_range();
                    // EventRemainder::None
                    (UiRequest::None, WindowCtl::HexGrid(HexGridCtl::SlcRangeFull))
                }))
            )

            .element(TextBox::new([1.0, -1.0, 0.0], (-0.385, 0.35), 4.45, 
                    "Iters:", enamel::ui::C_ORANGE, "1", Box::new(|key_state, vk_code, kb_state, text_string| {
                        enamel::ui::key_into_string(key_state, vk_code, kb_state, text_string);

                        let parsed = text_string.trim().replace("k","000").replace("m","0000000").parse();

                        let remainder = match parsed {
                            Ok(i) => WindowCtl::SetCyIters(i),
                            Err(_) => WindowCtl::None,
                        };

                        (UiRequest::None, remainder)
                    } )
                )
                .mouse_event_handler(Box::new(|_, _| {
                    (UiRequest::KeyboardFocus(true), WindowCtl::None)
                } ))

            )

            .element(HexButton::new([1.0, -1.0, 0.0], (-0.57, 0.25), 1.8, 
                    "Cycle", enamel::ui::C_ORANGE)
                .mouse_event_handler(Box::new(|_, _| {
                    // window.control_tx.send(CyCtl::Iterate(window.iters_pending))
                    //     .expect("Iterate button");
                    (UiRequest::None, WindowCtl::CyIterate)
                }))
            )

            .element(HexButton::new([1.0, -1.0, 0.0], (-0.20, 0.25), 1.8, 
                    "Stop", enamel::ui::C_ORANGE)
                .mouse_event_handler(Box::new(|_, _| {                     
                    // window.control_tx.send(CyCtl::Stop)
                    //     .expect("Stop button");
                    (UiRequest::None, WindowCtl::CyCtl(CyCtl::Stop))
                }))
            )

            .element(HexButton::new([1.0, -1.0, 0.0], (-0.20, 0.07), 1.8, 
                    "Exit", enamel::ui::C_ORANGE)
                .mouse_event_handler(Box::new(|_, _| { 
                    // window.close_pending = true;
                    (UiRequest::None, WindowCtl::Closed)
                }))
            )            

            .init();

        let grid_dims = hex_grid.buffer.aff_out_grid_dims();

        // Main window data struct:
        let mut window = Window {
            cycle_status: Status::new(),
            area_info: area_info.clone(),
            // area_name: info.name,
            // tract_map: tract_map,
            stats: WindowStats::new(),
            close_pending: false,
            grid_dims: grid_dims,
            // cam_dst: 1.0,
            iters_pending: 1,
            control_tx: control_tx,
            result_rx: result_rx,
            hex_grid: hex_grid,
            // hex_grid_buf: hex_grid_buf,
            // hex_grid.buffer.is_clear: false,
        };

        // // Print some stuff:
        // println!("\n==================== Vibi Keyboard Bindings ===================\n\
        //     {mt}The following keys must be used with 'ctrl':\n\
        //     {mt}'Escape' or 'q' to quit.",
        //     mt = "    ");


        //////////////////////////////////////////////////////////////////////////
        ///////////////////// Primary Event & Rendering Loop /////////////////////
        //////////////////////////////////////////////////////////////////////////
        loop {
            // Get read for new input:
            ui.set_input_stale();

            // Create draw target and clear color and depth:
            let mut target = display.draw();
            target.clear_color_and_depth((0.030, 0.050, 0.080, 1.0), 1.0);

            // Check cycle status:
            window.recv_cycle_results();
            window.control_tx.send(CyCtl::RequestCurrentIter).unwrap();

            // Check input events:
            for ev in display.poll_events() {
                window.handle_event_remainder(ui.handle_event(ev));
            }

            // If the hex grid buffer is not clear, e.g. the last sample
            // request is still unwritten, clear it, if possible, by
            // attempting to write to the device vertex buffer.
            if !window.hex_grid.buffer.is_clear() {
                let is_clear = window.hex_grid.buffer.refresh_vertex_buf();
                window.hex_grid.buffer.set_clear(is_clear);
            }

            // If the hex grid buffer is now clear, send a new sample request
            // for the next frame.
            if window.hex_grid.buffer.is_clear() {
                window.control_tx.send(CyCtl::Sample(window.hex_grid.buffer.cur_slc_range(),
                    window.hex_grid.buffer.raw_states_vec())) .expect("Sample raw states");
                window.hex_grid.buffer.set_clear(false);
            }

            // Draw hex grid:
            window.hex_grid.draw(&mut target, window.stats.elapsed_ms(), &window.hex_grid.buffer);

            // Draw status text:
            status_text.draw(&mut target, &window.cycle_status, &window.stats, window.grid_dims,
                &window.area_info.name, window.hex_grid.camera_pos()[2]);

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

            ///////////////////////////////////////////////////////////
            ////////////////////////// DEBUG //////////////////////////
            ///////////////////////////////////////////////////////////            
                // if !ui.input_is_stale() {
                //     println!("##### Mouse position: {:?}", ui.mouse_state().position());
                // }
        }

        // Hide window when exiting.
        // [FIXME] TODO: Draw "Closing..." or something like that to the display instead.
        display.get_window().unwrap().hide();
    }

    fn recv_cycle_results(&mut self) {
        loop {
            match self.result_rx.try_recv() {
                Ok(cr) => {
                    match cr {
                        CyRes::CurrentIter(iter) => self.cycle_status.cur_cycle = iter,
                        CyRes::Status(cysts) => self.cycle_status = *cysts,
                        CyRes::AreaInfo(box info) => {
                            // let AreaInfo { area, aff_out_slc_range, tract_map } = info.clone();
                            self.area_info = info.clone();
                            // self.area_name = name;
                            self.hex_grid.buffer.set_default_slc_range(info.aff_out_slc_range.clone());
                            self.hex_grid.buffer.set_tract_map(info.tract_map);
                        },
                        // _ => (),
                    }
                },
                Err(_) => break,
            };
        }
    }

    fn handle_event_remainder(&mut self, rdr: WindowCtl) {
        match rdr {
            WindowCtl::MouseWheel(delta) => self.handle_mouse_wheel(delta),
            WindowCtl::Closed => self.close_pending = true,
            WindowCtl::CyCtl(ctl) => self.control_tx.send(ctl).unwrap(),
            WindowCtl::SetCyIters(i) => self.iters_pending = i,
            WindowCtl::CyIterate => self.control_tx.send(CyCtl::Iterate(self.iters_pending)).unwrap(),
            WindowCtl::HexGrid(ctl) => {
                match ctl {
                    HexGridCtl::SlcRangeDefault => self.hex_grid.buffer.use_default_slc_range(),
                    HexGridCtl::SlcRangeFull => self.hex_grid.buffer.use_full_slc_range(),
                }
                self.hex_grid.update_cam_pos();
            },
            _ => (),           
        }
    }

    /// Moves the camera position in our out (horizontal scrolling ignored).
    #[allow(dead_code)]
    pub fn handle_mouse_wheel(&mut self, scroll_delta: MouseScrollDelta) {
        let (hrz, vrt) = match scroll_delta {
            MouseScrollDelta::LineDelta(h, v) => (h * 0.01, v * 0.01),
            MouseScrollDelta::PixelDelta(x, y) => (x * 0.001, y * 0.001),
        };

        self.hex_grid.move_camera([0.0, 0.0, vrt]);
        let _ = hrz;

        // let vrt_delta = vrt * -0.01;
        // let new_cam_dst = self.cam_dst + vrt_delta;
        // let new_dst_valid = (new_cam_dst >= 0.00 && new_cam_dst <= 2.99) as i32 as f32;
        // self.cam_dst += vrt_delta * new_dst_valid;
    }

    #[allow(dead_code, unused_variables)]
    pub fn handle_mouse_input(&mut self, button_state: ElementState, button: MouseButton) {

    }

    #[allow(dead_code, unused_variables)]
    pub fn handle_mouse_moved(&mut self, mouse_state: &MouseState) {

    }
}
