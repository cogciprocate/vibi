use std::sync::mpsc::{Receiver, Sender, TryRecvError, SendError};
use time::{self, Timespec, Duration};
// use glium::{self, Surface, DisplayBuild};
use glium::{self, glutin, Surface};
// use glium::backend::glutin::Display;
// use cycle::{CyCmd, CyRes, Status as CyStatus, AreaInfo};
use bismit::{SamplerKind, SamplerBufferKind};
use bismit::flywheel::{Command, Request, Response, Status, AreaInfo};
use window::{HexGrid, StatusText};
use enamel::{ui, Pane, EventRemainder, UiRequest, TextBox, HexButton, ElementState,
    MouseButton, MouseScrollDelta, SetMouseFocus, Event, WindowEvent};


#[derive(Clone, Debug)]
pub enum HexGridCtl {
    SlcRangeDefault,
    SlcRangeFull,
}


#[derive(Clone, Debug)]
pub enum WindowCtl {
    None,
    Event(Event),
    HexGrid(HexGridCtl),
    SetCyIters(u32),
    CyIterate,
    CyCmd(Command),
    Close,
}

impl EventRemainder for WindowCtl {
    fn event(event: Event) -> Self {
        WindowCtl::Event(event)
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
    pub cycle_in_progress: bool,
    pub area_info: AreaInfo,
    pub stats: WindowStats,
    pub close_pending: bool,
    pub grid_dims: (u32, u32),
    pub iters_pending: u32,
    pub command_tx: Sender<Command>,
    pub request_tx: Sender<Request>,
    pub response_rx: Receiver<Response>,
    pub hex_grid: HexGrid<'d>,
    pub has_mouse_focus: bool,
    pub mouse_pos: (i32, i32),
    pub dragging: Option<(i32, i32)>,
}

impl<'d> Window<'d> {
    pub fn open(command_tx: Sender<Command>, request_tx: Sender<Request>,
                response_rx: Receiver<Response>) {

        // println!("VIBI: 0");

        // Get initial area info:
        request_tx.send(Request::AreaInfo)
            .expect("Error requesting current area name");
        command_tx.send(Command::None).unwrap();

        let area_info;
        loop {
            println!("VIBI: Attempting to receive area info...");
            match response_rx.recv().expect("Current area name reception error") {
                Response::AreaInfo(info) => {
                    area_info = *info;
                    break;
                },
                _ => (),
            };
        }

        println!("VIBI: Creating window...");

        // let display = glium::glutin::WindowBuilder::new()
        //     .with_depth_buffer(24)
        //     .with_dimensions(1400, 800)
        //     .with_title("Vibi".to_string())
        //     .with_multisampling(8)
        //     // Disabled for development ->> .with_gl_robustness(glium::glutin::Robustness::NoError)
        //     .with_vsync()
        //     // .with_transparency(true)
        //     // .with_fullscreen(glium::glutin::get_primary_monitor())
        //     .build_glium().unwrap();

        let mut events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_dimensions(1600, 900)
            .with_title("Vibi".to_string());
        let context = glutin::ContextBuilder::new()
            .with_depth_buffer(24)
            .with_vsync(true)
            .with_multisampling(8);
        let display = glium::Display::new(window, context, &events_loop).unwrap();

        // Hex grid:
        let hex_grid = HexGrid::new(&display, area_info.clone());
        let grid_dims = hex_grid.buffer.aff_out_grid_dims();

        // Status text UI element (fps & grid side):
        let status_text = StatusText::new(&display);

        // Primary user interface elements:
        let mut ui = Pane::new(&display)
            .element(HexButton::new(ui::BOTTOM_RIGHT, (-0.57, 0.45), 1.8,
                    "View Output", ui::C_ORANGE)
                .mouse_event_handler(Box::new(|_, _| {
                    (UiRequest::None, WindowCtl::HexGrid(HexGridCtl::SlcRangeDefault))
                }))
            )

            .element(HexButton::new(ui::BOTTOM_RIGHT, (-0.20, 0.45), 1.8,
                    "View All", ui::C_ORANGE)
                .mouse_event_handler(Box::new(|_, _| {
                    (UiRequest::None, WindowCtl::HexGrid(HexGridCtl::SlcRangeFull))
                }))
            )

            .element(TextBox::new(ui::BOTTOM_RIGHT, (-0.385, 0.35), 4.45,
                    "Iters:", ui::C_ORANGE, "1m")
                .keyboard_event_handler(Box::new(|key_state, vk_code, kb_state,
                    text_string| {
                        ui::key_into_string(key_state, vk_code, kb_state, text_string);

                        let parsed = text_string.trim().replace("k","000")
                            .replace("m","000000").parse();

                        let remainder = match parsed {
                            Ok(i) => WindowCtl::SetCyIters(i),
                            Err(_) => WindowCtl::None,
                        };

                        (UiRequest::None, remainder)
                    } )
                )
                .mouse_event_handler(Box::new(|_, _| {
                    (UiRequest::KeyboardFocus(true), WindowCtl::None)
                })

            ))

            .element(HexButton::new(ui::BOTTOM_RIGHT, (-0.57, 0.25), 1.8,
                    "Cycle", ui::C_ORANGE)
                .mouse_event_handler(Box::new(|_, _| {
                    (UiRequest::None, WindowCtl::CyIterate)
                }))
            )

            .element(HexButton::new(ui::BOTTOM_RIGHT, (-0.20, 0.25), 1.8,
                    "Stop", ui::C_ORANGE)
                .mouse_event_handler(Box::new(|_, _| {
                    (UiRequest::None, WindowCtl::CyCmd(Command::Stop))
                }))
            )

            .element(HexButton::new(ui::BOTTOM_RIGHT, (-0.20, 0.07), 1.8,
                    "Exit", ui::C_ORANGE)
                .mouse_event_handler(Box::new(|_, _| {
                    (UiRequest::None, WindowCtl::Close)
                }))
            )

            .init();

        // Main window data struct:
        let mut window = Window {
            cycle_status: Status::new(),
            cycle_in_progress: false,
            area_info: area_info.clone(),
            stats: WindowStats::new(),
            close_pending: false,
            grid_dims: grid_dims,
            iters_pending: 1000000,
            command_tx: command_tx,
            request_tx: request_tx,
            response_rx: response_rx,
            hex_grid: hex_grid,
            mouse_pos: (0, 0),
            has_mouse_focus: true,
            dragging: None,
        };

        // // Print some stuff:
        // println!("\n==================== Vibi Keyboard Bindings ===================\n\
        //     {mt}The following keys must be used with 'ctrl':\n\
        //     {mt}'Escape' or 'q' to quit.",
        //     mt = "    ");

        // Returns true if any send errors are received.
        fn handle_init_sends<T>(res: Result<(), SendError<T>>, window: &mut Window) {
            if let Err(_) = res {
                window.close_pending = window.close_pending & true;
            }
        }

        // Send initial requests:
        handle_init_sends(window.request_tx.send(Request::CurrentIter), &mut window);
        handle_init_sends(window.command_tx.send(Command::None), &mut window);
        window.recv_cycle_results(true);

        handle_init_sends(window.request_tx.send(Request::Status), &mut window);
        handle_init_sends(window.command_tx.send(Command::None), &mut window);
        window.recv_cycle_results(true);

        // AxonSpace -> HexGridBuffer sampler:
        handle_init_sends(window.request_tx.send(Request::Sampler {
                area_name: window.area_info.name.clone(),
                kind: SamplerKind::Axons(None),
                buffer_kind: SamplerBufferKind::Single,
                backpressure: false,
            }), &mut window);
        handle_init_sends(window.command_tx.send(Command::None), &mut window);
        window.recv_cycle_results(true);

        if window.close_pending { println!("Send error during vibi window init."); }

        //////////////////////////////////////////////////////////////////////////
        ///////////////////// Primary Event & Rendering Loop /////////////////////
        //////////////////////////////////////////////////////////////////////////
        loop {
            // Create draw target and clear color and depth:
            let mut target = display.draw();
            target.clear_color_and_depth((0.030, 0.050, 0.080, 1.0), 1.0);

            // Get read for new input:
            ui.set_input_stale();

            // // Check input events:
            // for ev in display.poll_events() {
            //     window.handle_event_remainder(ui.handle_event(ev));
            // }

            // Check input events:
            events_loop.poll_events(|ev| {
                window.handle_event_remainder(ui.handle_event(ev));
            });

            // Check the results channel and determine if the cycle process
            // has caught up to this window before sending new requests.
            if window.recv_cycle_results(false) {
                // Early exit (avoids a channel panic):
                if window.close_pending {
                    target.finish().unwrap();
                    window.command_tx.send(Command::Exit).ok();
                    break;
                }
            }

            // AxonSpace -> HexGridBuffer sampler:
            window.hex_grid.buffer.refresh_vertex_buf();

            if window.cycle_in_progress {
                // Check current iterator for next frame:
                window.request_tx.send(Request::CurrentIter).unwrap();
            }

            // Increment our counters:
            let elapsed_ms = window.stats.elapsed_ms();
            window.stats.incr();

            // Draw hex grid:
            window.hex_grid.draw(&mut target, elapsed_ms);

            // Draw status text:
            status_text.draw(&mut target, &window.cycle_status, &window.stats, window.grid_dims,
            &window.area_info.name, window.hex_grid.camera_pos()[2], window.hex_grid.top_right_scene,
            window.hex_grid.cam_pos_raw(), window.hex_grid.tract_map());

            // Draw UI:
            ui.draw(&mut target);

            // Swap buffers:
            target.finish().unwrap();

            // Clean up and exit if necessary:
            if window.close_pending {
                window.command_tx.send(Command::Exit).ok();
                break;
            }
        }

        // Hide window when exiting.
        // [FIXME] TODO: Draw "Closing..." or something like that to the display instead.
        display.gl_window().hide();
    }

    fn handle_response(&mut self, response: Response) {
        match response {
            Response::CurrentIter(iter) => self.cycle_status.cur_cycle.0 = iter,
            Response::Status(cysts) => {
                self.cycle_status = *cysts;
                self.cycle_in_progress = self.cycle_status.cycling;
                // println!("Cycle Status: cycling: {}", self.cycle_in_progress);
            },
            Response::AreaInfo(info) => {
                let info = *info;
                self.area_info = info.clone();
                // FIXME: Reimplement handling of switching visible slices.
                // self.hex_grid.buffer.set_default_slc_ids(info.aff_out_slc_ids.clone());
                self.hex_grid.buffer.set_tract_map(info.tract_map);
            },
            Response::Exiting => self.close_pending = true,
            Response::Sampler(tract_rx) => {
                self.hex_grid.buffer.set_tract_buffer(tract_rx);
            },
            _ => (),
        }
    }

    fn recv_cycle_results(&mut self, block: bool) -> bool {
        let mut any_recvd = false;

        loop {
            if block {
                let response = self.response_rx.recv().unwrap();
                self.handle_response(response);
                any_recvd = true;
                break;
            } else {
                match self.response_rx.try_recv() {
                    Ok(response) => {
                        self.handle_response(response);
                        any_recvd = true;
                    },
                    Err(e) => match e {
                        TryRecvError::Empty => break,
                        TryRecvError::Disconnected => {
                            self.cycle_in_progress = false;
                            self.close_pending = true;
                            break;
                        },
                    },
                }
            }
        }

        // // DEBUG:
        // println!("recv_cycle_results(): any_recvd: {}", any_recvd);

        any_recvd
    }

    fn handle_event_remainder(&mut self, rdr: WindowCtl) {
        match rdr {
            WindowCtl::None => (),
            WindowCtl::Event(event) => match event {
                Event::WindowEvent { window_id: _, event: win_event } => {
                    match win_event {
                        // Event::KeyboardInput(state, _, v_code) => ()
                        //     println!("Key: {:?} has been {:?}", ui::map_vkc(v_code), state),
                        WindowEvent::MouseMoved { device_id: _, position } => {
                            self.handle_mouse_moved(position)
                        },
                        WindowEvent::MouseWheel { device_id: _, delta, phase: _ } => {
                            self.handle_mouse_wheel(delta)
                        },
                        WindowEvent::MouseInput { device_id: _, state, button } => {
                            self.handle_mouse_input(state, button)
                        },
                        WindowEvent::Touch(touch) => println!("Touch recieved: {:?}", touch),
                        WindowEvent::Closed => self.close_pending = true,
                        _ => (),
                    }
                },
                _ => (),
            },
            WindowCtl::CyCmd(cmd) => {
                // If `Stop`/`Exit` is being sent or if any other command is
                // being sent while the cycle is in progress, set
                // `cycle_in_progress` false.
                match cmd.clone() {
                    Command::Stop | Command::Exit => self.cycle_in_progress = false,
                    _ => if self.cycle_in_progress { self.cycle_in_progress = false; },
                }

                self.command_tx.send(cmd).unwrap();
            },
            WindowCtl::SetCyIters(i) => self.iters_pending = i,
            WindowCtl::CyIterate => {
                self.command_tx.send(Command::Iterate(self.iters_pending)).unwrap();
                self.cycle_in_progress = true;
            },
            WindowCtl::HexGrid(cmd) => {
                match cmd {
                    HexGridCtl::SlcRangeDefault => self.hex_grid.buffer.use_default_slc_range(),
                    HexGridCtl::SlcRangeFull => self.hex_grid.buffer.use_full_slc_range(),
                }
                self.hex_grid.update_cam_pos();
            },
            WindowCtl::Close => self.close_pending = true,
            // _ => (),
        }
    }

    /// Moves the camera position in our out (horizontal scrolling ignored).
    #[allow(dead_code)]
    fn handle_mouse_wheel(&mut self, scroll_delta: MouseScrollDelta) {
        let (hrz, vrt) = match scroll_delta {
            MouseScrollDelta::LineDelta(h, v) => (h * 10.0, v * 10.0),
            MouseScrollDelta::PixelDelta(x, y) => (x, y),
        };
        let _ = hrz;

        self.hex_grid.zoom_camera(vrt);
    }

    fn handle_mouse_moved(&mut self, pos: (f64, f64)) {
        let pos = (pos.0 as i32, pos.1 as i32);
        self.mouse_pos = pos;

        if let Some(ref mut start_pos) = self.dragging {
            let delta = (pos.0 - start_pos.0, pos.1 - start_pos.1);
            self.hex_grid.move_camera(delta);
            *start_pos = pos;
        }
    }

    #[allow(dead_code, unused_variables)]
    fn handle_mouse_input(&mut self, button_state: ElementState, button: MouseButton) {
        match button {
            MouseButton::Left => {
                match button_state {
                    ElementState::Pressed => self.dragging = Some(self.mouse_pos),
                    ElementState::Released => self.dragging = None,
                }
            },
            _ => (),
        }
        // println!("WINDOW::HANDLE_MOUSE_INPUT(): focus: {}, dragging: {:?}", self.has_mouse_focus, self.dragging);
    }
}

impl<'d> SetMouseFocus for Window<'d> {
    fn set_mouse_focus(&mut self, focus: bool) {
        self.has_mouse_focus = focus;
        // println!("WINDOW::SET_MOUSE_FOCUS(): Setting focus to: {}, dragging: {:?}", focus, self.dragging);
    }
}