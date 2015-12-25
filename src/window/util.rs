use glium::glutin::{VirtualKeyCode, ElementState};
use glium::glutin::VirtualKeyCode::*;
use window::{util, KeyboardState};


pub fn key_into_string(key_state: ElementState, vk_code: Option<VirtualKeyCode>, kb_state: &KeyboardState, 
			string: &mut String) 
{
	if let ElementState::Pressed = key_state {
		match vk_code {
			Some(Back) => {
				string.pop();
			},

			_ => {
				if let Some(mut c) = util::map_vkc(vk_code) {					
					if kb_state.shift { c = c.to_uppercase().next().unwrap_or(c); }
					string.push(c);				
				}
			},
		}
	}
}

/// Brighten or darken a single color component. 
///
/// An `amount` of `-1.0` will completely minimize (0.0) and `1.0` will completely maximize (1.0).
pub fn adjust_ccmp(component: f32, amount: f32) -> f32 {
	// Clamp amount in [-1.0, 1.0] and component in [0.0, 1.0]:
	let amount = if amount > 1.0 { 1.0 } else if amount < -1.0 { -1.0 } else { amount };
	let component = if component > 1.0 { 1.0 } else if component < 0.0 { 0.0 } else { component };

	if amount >= 0.0 {
		let until_max = 1.0 - component;
		component + (until_max * amount)
	} else {
		let until_min = component;
		component - (-amount * until_min)
	}
}

pub fn adjust_color(color: [f32; 4], amount: f32) -> [f32; 4] {
	[
		adjust_ccmp(color[0], amount),
		adjust_ccmp(color[1], amount),
		adjust_ccmp(color[2], amount),
		color[3],
	]
}

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


#[cfg(test)]
mod tests {
	use super::*;

	// TODO: Need more test variations! Haven't wasted enough time on this yet!
	#[test]
	fn test_adjust_ccmp() {
		assert_eq!(adjust_ccmp(1.0, 1.0), 1.0);
		assert_eq!(adjust_ccmp(5.0, 5.0), 1.0);
		assert_eq!(adjust_ccmp(1.5, 1.5), 1.0);

		assert_eq!(adjust_ccmp(1.0, -1.0), 0.0);
		assert_eq!(adjust_ccmp(-5.0, -5.0), 0.0);
		assert_eq!(adjust_ccmp(5.0, -5.0), 0.0);

		assert_eq!(adjust_ccmp(1.0, 0.0), 1.0);

		assert_eq!(adjust_ccmp(0.5, 1.0), 1.0);
		assert_eq!(adjust_ccmp(0.5, -1.0), 0.0);
		assert_eq!(adjust_ccmp(0.5, 0.0), 0.5);

		assert_eq!(adjust_ccmp(0.5, 0.5), 0.75);
		assert_eq!(adjust_ccmp(0.5, 0.05), 0.525);
		assert_eq!(adjust_ccmp(0.5, -0.5), 0.25);
		assert_eq!(adjust_ccmp(0.5, -0.05), 0.475);
	}
}
