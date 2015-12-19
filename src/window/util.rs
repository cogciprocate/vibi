use glium::glutin::VirtualKeyCode;
use glium::glutin::VirtualKeyCode::*;

// [FIXME]: TODO: 
// - Consider using a hashmap? Could be more efficient.
pub fn map_vkc(vkc: Option<VirtualKeyCode>) -> Option<char> {
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
