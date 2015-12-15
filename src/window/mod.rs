
pub use self::win_stats::{WinStats};
pub use self::status_text::{StatusText};
pub use self::hex_grid::{HexGrid};
pub use self::ui::{Ui};
pub use self::ui_element::{UiElement};
pub use self::ui_vertex::{UiVertex};

mod win_stats;
mod status_text;
mod hex_grid;
mod ui;
mod ui_element;
mod ui_vertex;
// mod conrod;
// mod window_grid;
pub mod window_main;

// const C_PINK: [f32; 3] = [0.9882, 0.4902, 0.7059];
const C_ORANGE: [f32; 3] = [0.9607, 0.4745, 0.0];
