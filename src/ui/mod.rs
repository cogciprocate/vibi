#![allow(dead_code)]

mod element;
mod pane;
mod shape_2d;
mod text_properties;
mod vertex;

pub use self::element::{UiElement, UiElementBorder, UiElementKind};
pub use self::pane::UiPane;
pub use self::shape_2d::UiShape2d;
pub use self::text_properties::TextProperties;
pub use self::vertex::UiVertex;


pub const C_PINK: [f32; 4] = [0.990, 0.490, 0.700, 1.0];
pub const C_ORANGE: [f32; 4] = [0.960, 0.400, 0.0, 1.0];
pub const C_DARK_ORANGE: [f32; 4] = [0.384, 0.080, 0.0, 1.0]; 
pub const C_BLUE: [f32; 4] = [0.204, 0.396, 0.643, 1.0];
pub const C_BLACK: [f32; 4] = [0.001, 0.001, 0.001, 1.0];

