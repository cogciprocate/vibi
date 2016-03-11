#![allow(dead_code)]

mod window_stats;
mod status_text;
// mod mouse_state;
// mod keyboard_state;
mod controls;
mod window;
mod tract_buffer;

pub use self::window_stats::WindowStats;
pub use self::status_text::StatusText;
pub use self::controls::HexGrid;
// pub use self::mouse_state::MouseState;
pub use self::controls::{Button, HexButton, TextBox};
pub use self::window::Window;
// pub use self::keyboard_state::KeyboardState;
pub use self::tract_buffer::{TractBuffer, StateVertex};


// pub const INIT_GRID_SIZE: u32 = 64;
pub const MAX_GRID_SIZE: u32 = 8192;
