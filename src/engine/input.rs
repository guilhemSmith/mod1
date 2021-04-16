use glam::Vec2;
use glutin::dpi::PhysicalPosition;
use glutin::event::{ElementState, KeyboardInput, MouseButton};
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

#[allow(dead_code)]
pub enum ClickButton {
	Left,
	Right,
}

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

	fn update(&mut self, new_state: ElementState) {
		let pressed = new_state == ElementState::Pressed;
		self.just_changed = !self.just_changed && pressed != self.pressed;
		self.pressed = pressed;
	}
}

struct MouseState {
	pos: Vec2,
	rel: Vec2,
	left_click: KeyState,
	right_click: KeyState,
}

pub struct Inputs {
	active: bool,
	keys_states: HashMap<u32, KeyState>,
	mouse_state: MouseState,
}

impl Inputs {
	pub fn new() -> Self {
		Inputs {
			active: false,
			keys_states: HashMap::new(),
			mouse_state: MouseState {
				pos: Vec2::ZERO,
				rel: Vec2::ZERO,
				left_click: KeyState::new(false),
				right_click: KeyState::new(false),
			},
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

	#[allow(dead_code)]
	pub fn is_click_pressed(&self, button: ClickButton) -> bool {
		match button {
			ClickButton::Left => self.mouse_state.left_click.pressed,
			ClickButton::Right => self.mouse_state.right_click.pressed,
		}
	}

	#[allow(dead_code)]
	pub fn is_click_released(&self, button: ClickButton) -> bool {
		match button {
			ClickButton::Left => !self.mouse_state.left_click.pressed,
			ClickButton::Right => !self.mouse_state.right_click.pressed,
		}
	}

	#[allow(dead_code)]
	pub fn is_click_just_pressed(&self, button: ClickButton) -> bool {
		match button {
			ClickButton::Left => {
				self.mouse_state.left_click.pressed && self.mouse_state.left_click.just_changed
			}
			ClickButton::Right => {
				self.mouse_state.right_click.pressed && self.mouse_state.right_click.just_changed
			}
		}
	}

	#[allow(dead_code)]
	pub fn is_click_just_released(&self, button: ClickButton) -> bool {
		match button {
			ClickButton::Left => {
				!self.mouse_state.left_click.pressed && self.mouse_state.left_click.just_changed
			}
			ClickButton::Right => {
				!self.mouse_state.right_click.pressed && self.mouse_state.right_click.just_changed
			}
		}
	}

	#[allow(dead_code)]
	pub fn mouse_rel(&self) -> Vec2 {
		self.mouse_state.rel
	}

	pub fn update(&mut self) {
		if self.active {
			for (_key, state) in self.keys_states.iter_mut() {
				state.just_changed = false;
			}
			self.mouse_state.left_click.just_changed = false;
			self.mouse_state.right_click.just_changed = false;
			self.mouse_state.rel = Vec2::ZERO;
			self.active = false;
		}
	}

	pub fn store_key(&mut self, input: KeyboardInput) {
		self.active = true;
		match self.keys_states.get_mut(&input.scancode) {
			Some(state) => {
				state.update(input.state);
			}
			None => {
				let new_state = KeyState::new(input.state == ElementState::Pressed);
				self.keys_states.insert(input.scancode, new_state);
			}
		}
	}

	pub fn store_click(&mut self, state: ElementState, button: MouseButton) {
		self.active = true;
		match button {
			MouseButton::Left => self.mouse_state.left_click.update(state),
			MouseButton::Right => self.mouse_state.right_click.update(state),
			_ => {}
		}
	}

	pub fn store_motion(&mut self, position: PhysicalPosition<f64>) {
		self.active = true;
		let new_pos = Vec2::new(position.x as f32, position.y as f32);
		let offset = new_pos - self.mouse_state.pos;
		self.mouse_state.rel += offset;
		self.mouse_state.pos = new_pos;
	}
}
