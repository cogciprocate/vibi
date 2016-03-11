#![allow(dead_code)]

mod element;
mod pane;
mod shape_2d;
mod text_properties;
mod vertex;
mod keyboard_state;
mod mouse_state;

pub use self::mouse_state::MouseState;
pub use self::keyboard_state::KeyboardState;
pub use self::element::{Element, ElementBorder, ElementKind, ElementText};
pub use self::pane::Pane;
pub use self::shape_2d::Shape2d;
// pub use self::text_properties::TextProperties;
pub use self::vertex::Vertex;
pub use self::types::{MouseInputHandler, KeyboardInputHandler};
pub use self::enums::{TextAlign, MouseInputEventResult, KeyboardInputEventResult, HandlerOption};
pub use self::functions::{ key_into_string };
// pub use self::traits::HandlerWindow;

pub const C_PINK: [f32; 4] = [0.990, 0.490, 0.700, 1.0];
pub const C_ORANGE: [f32; 4] = [0.960, 0.400, 0.0, 1.0];
pub const C_DARK_ORANGE: [f32; 4] = [0.384, 0.080, 0.0, 1.0]; 
pub const C_BLUE: [f32; 4] = [0.204, 0.396, 0.643, 1.0];
pub const C_BLACK: [f32; 4] = [0.001, 0.001, 0.001, 1.0];
pub const SUBDEPTH: f32 = -0.015625;
pub const SUBSUBDEPTH: f32 = 0.000244140625;


// Implement this one day. Just a type which has some data for the handlers to use.
mod traits {
    pub trait HandlerWindow {

    }
}


mod types {
    use glium::glutin::{ElementState, MouseButton, VirtualKeyCode};

    use ui::{MouseInputEventResult, KeyboardInputEventResult, KeyboardState, };
    use window::Window;

    pub type MouseInputHandler = Box<FnMut(ElementState, MouseButton, 
        &mut Window) -> MouseInputEventResult>;
    pub type KeyboardInputHandler = Box<FnMut(ElementState, Option<VirtualKeyCode>, &KeyboardState, &mut String,
        &mut Window) -> KeyboardInputEventResult>;
}


mod enums {
    use std::fmt::{Debug, Formatter, Error};

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
}


mod functions {
    use glium::glutin::{VirtualKeyCode, ElementState};
    // use glium::glutin::VirtualKeyCode::*;
    use ui::KeyboardState;

    pub fn key_into_string(key_state: ElementState, vk_code: Option<VirtualKeyCode>, kb_state: &KeyboardState, 
            string: &mut String) 
    {
        if let ElementState::Pressed = key_state {
            match vk_code {
                Some(VirtualKeyCode::Back) => {
                    string.pop();
                },

                _ => {
                    if let Some(mut c) = map_vkc(vk_code) {                    
                        if kb_state.shift { c = c.to_uppercase().next().unwrap_or(c); }
                        string.push(c);                
                    }
                },
            }
        }
    }

    // [FIXME]: TODO: 
    // - Consider using a hashmap? Could be more efficient.
    pub fn map_vkc(vkc: Option<VirtualKeyCode>) -> Option<char> {
        use glium::glutin::VirtualKeyCode::*;

        if let Some(vkc) = vkc { 
            match vkc {
                Key1 | Numpad0 => Some('1'),
                Key2 | Numpad1 => Some('2'),
                Key3 | Numpad2 => Some('3'),
                Key4 | Numpad3 => Some('4'),
                Key5 | Numpad4 => Some('5'),
                Key6 | Numpad5 => Some('6'),
                Key7 | Numpad6 => Some('7'),
                Key8 | Numpad7 => Some('8'),
                Key9 | Numpad8 => Some('9'),
                Key0 | Numpad9 => Some('0'),    

                A => Some('a'),
                B => Some('b'),
                C => Some('c'),
                D => Some('d'),
                E => Some('e'),
                F => Some('f'),
                G => Some('g'),
                H => Some('h'),
                I => Some('i'),
                J => Some('j'),
                K => Some('k'),
                L => Some('l'),
                M => Some('m'),
                N => Some('n'),
                O => Some('o'),
                P => Some('p'),
                Q => Some('q'),
                R => Some('r'),
                S => Some('s'),
                T => Some('t'),
                U => Some('u'),
                V => Some('v'),
                W => Some('w'),
                X => Some('x'),
                Y => Some('y'),
                Z => Some('z'),

                Space => Some(' '),

                _ => None

            }
        } else {
            None
        }
    }
}
