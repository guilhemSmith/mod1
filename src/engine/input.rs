use glutin::event::{ElementState, KeyboardInput};
use std::collections::HashMap;

#[allow(dead_code)]
pub const K_ESC: u32 = 1;
#[allow(dead_code)]
pub const K_SPACE: u32 = 57;
#[allow(dead_code)]
pub const K_ENTER: u32 = 28;
#[allow(dead_code)]
pub const K_UP: u32 = 103;
#[allow(dead_code)]
pub const K_DOWN: u32 = 108;
#[allow(dead_code)]
pub const K_LEFT: u32 = 105;
#[allow(dead_code)]
pub const K_RIGHT: u32 = 106;

struct KeyState {
	pressed: bool,
	just_changed: bool,
}

impl KeyState {
	fn new(state: bool) -> Self {
		KeyState {
			pressed: state,
			just_changed: true,
		}
	}
}

pub struct Inputs {
	active: bool,
	keys_states: HashMap<u32, KeyState>,
}

impl Inputs {
	pub fn new() -> Self {
		Inputs {
			active: false,
			keys_states: HashMap::new(),
		}
	}

	#[allow(dead_code)]
	pub fn is_pressed(&self, code: u32) -> bool {
		match self.keys_states.get(&code) {
			Some(state) => state.pressed,
			None => false,
		}
	}

	#[allow(dead_code)]
	pub fn is_released(&self, code: u32) -> bool {
		match self.keys_states.get(&code) {
			Some(state) => !state.pressed,
			None => true,
		}
	}

	#[allow(dead_code)]
	pub fn is_just_pressed(&self, code: u32) -> bool {
		match self.keys_states.get(&code) {
			Some(state) => state.pressed && state.just_changed,
			None => false,
		}
	}

	#[allow(dead_code)]
	pub fn is_just_released(&self, code: u32) -> bool {
		match self.keys_states.get(&code) {
			Some(state) => !state.pressed && state.just_changed,
			None => false,
		}
	}

	pub fn update(&mut self) {
		if self.active {
			for (_key, state) in self.keys_states.iter_mut() {
				state.just_changed = false;
			}
			self.active = false;
		}
	}

	pub fn store_key(&mut self, input: KeyboardInput) {
		self.active = true;
		// println!("{}", input.scancode);
		match self.keys_states.get_mut(&input.scancode) {
			Some(state) => {
				let pressed = input.state == ElementState::Pressed;
				state.just_changed = pressed != state.pressed;
				state.pressed = pressed;
			}
			None => {
				let new_state = KeyState::new(input.state == ElementState::Pressed);
				self.keys_states.insert(input.scancode, new_state);
			}
		}
	}
}
