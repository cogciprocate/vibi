#![allow(dead_code)]

pub use self::window_stats::WindowStats;
pub use self::status_text::StatusText;
pub use self::hex_grid::HexGrid;
pub use self::ui_pane::UiPane;
pub use self::ui_element::{UiElement, UiElementKind};
pub use self::ui_vertex::UiVertex;
pub use self::ui_shape_2d::UiShape2d;
pub use self::mouse_state::MouseState;
pub use self::controls::{Button, HexButton, TextBox};
pub use self::main_window::MainWindow;
pub use self::text_properties::TextProperties;
pub use self::keyboard_state::KeyboardState;

mod window_stats;
mod status_text;
mod hex_grid;
mod ui_pane;
mod ui_element;
mod ui_vertex;
mod text_properties;
mod mouse_state;
mod keyboard_state;
mod controls;
mod main_window;
pub mod util;
pub mod ui_shape_2d;
// pub mod conrod;
// mod window_grid;
// pub mod window_main;

// use std::sync::mpsc::{Receiver, Sender};
// use loop_cycles::{CyCtl, CySts};
// use glium::{self, DisplayBuild, Surface};
use std::fmt::{Debug, Formatter, Error};
use glium::glutin::{ElementState, MouseButton, VirtualKeyCode};


pub const C_PINK: [f32; 4] = [0.990, 0.490, 0.700, 1.0];
pub const C_ORANGE: [f32; 4] = [0.960, 0.400, 0.0, 1.0];
pub const C_DARK_ORANGE: [f32; 4] = [0.384, 0.080, 0.0, 1.0]; 
pub const C_BLUE: [f32; 4] = [0.204, 0.396, 0.643, 1.0];
pub const C_BLACK: [f32; 4] = [0.001, 0.001, 0.001, 1.0];


// pub const INIT_GRID_SIZE: u32 = 64;
pub const MAX_GRID_SIZE: u32 = 8192;

pub const SUBDEPTH: f32 = -0.015625;
pub const SUBSUBDEPTH: f32 = 0.000244140625;

pub type MouseInputHandler = Box<FnMut(ElementState, MouseButton, 
	&mut MainWindow) -> MouseInputEventResult>;
pub type KeyboardInputHandler = Box<FnMut(ElementState, Option<VirtualKeyCode>, &KeyboardState, &mut String,
	&mut MainWindow) -> KeyboardInputEventResult>;


pub enum TextAlign {
	Center,
	Left,
	Right,
}

pub enum MouseInputEventResult {
	None,
	RequestKeyboardFocus(bool),
	RequestRedraw,
}

pub enum KeyboardInputEventResult {
	None,
	RequestRedraw,
}


pub enum HandlerOption<T> {
	None,
	Fn(T),
	Sub(usize),	
}

impl<T> HandlerOption<T> {
	pub fn is_some(&self) -> bool {
		if let &HandlerOption::None = self {
			false
		} else {
			true
		}
	}
}

impl<T> Debug for HandlerOption<T> {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		match self {
			&HandlerOption::Fn(_) => write!(f, "HandlerOption::Fn(_)"),
			&HandlerOption::Sub(idx) => write!(f, "HandlerOption::Sub({})", idx),
			&HandlerOption::None => write!(f, "HandlerOption::None"),
		}
	}
}



// /// Shifts a list of indices by `shift_by`.
// pub fn shift_indices(indices: &mut Vec<u16>, shift_by: u16) {
// 	for index in indices.iter_mut() {
// 		*index += shift_by;
// 	}
// }
