pub use self::window_stats::WindowStats;
pub use self::status_text::StatusText;
pub use self::hex_grid::HexGrid;
pub use self::ui_pane::UiPane;
pub use self::ui_element::UiElement;
pub use self::ui_vertex::UiVertex;
pub use self::mouse_state::MouseState;
pub use self::controls::{HexButton, TextBox};
pub use self::main_window::MainWindow;

// use std::sync::mpsc::{Receiver, Sender};
// use loop_cycles::{CyCtl, CySts};
// use glium::{self, DisplayBuild, Surface};

mod window_stats;
mod status_text;
mod hex_grid;
mod ui_pane;
mod ui_element;
mod ui_vertex;
mod mouse_state;
mod controls;
mod main_window;
pub mod shapes;
// pub mod conrod;
// mod window_grid;
// pub mod window_main;

pub const C_PINK: [f32; 3] = [0.990, 0.490, 0.700];
pub const C_ORANGE: [f32; 3] = [0.960, 0.400, 0.0];

pub const INIT_GRID_SIZE: u32 = 64;
pub const MAX_GRID_SIZE: u32 = 8192;

#[allow(dead_code)]
pub enum TextAlign {
	Center,
	Left,
	Right,
}


// /// Shifts a list of indices by `shift_by`.
// pub fn shift_indices(indices: &mut Vec<u16>, shift_by: u16) {
// 	for index in indices.iter_mut() {
// 		*index += shift_by;
// 	}
// }
