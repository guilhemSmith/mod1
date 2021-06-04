use glam::Vec2;
use glutin::dpi::PhysicalPosition;
use glutin::event::{ElementState, KeyboardInput, MouseButton, VirtualKeyCode};
use std::collections::HashMap;

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
	keys_states: HashMap<KeyCode, KeyState>,
	mouse_state: MouseState,
	wheel_delta: f32,
}

#[allow(dead_code)]
pub type KeyCode = VirtualKeyCode;

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
			wheel_delta: 0.0,
		}
	}

	#[allow(dead_code)]
	pub fn is_pressed(&self, code: KeyCode) -> bool {
		match self.keys_states.get(&code) {
			Some(state) => state.pressed,
			None => false,
		}
	}

	#[allow(dead_code)]
	pub fn is_released(&self, code: KeyCode) -> bool {
		match self.keys_states.get(&code) {
			Some(state) => !state.pressed,
			None => true,
		}
	}

	#[allow(dead_code)]
	pub fn is_just_pressed(&self, code: KeyCode) -> bool {
		match self.keys_states.get(&code) {
			Some(state) => state.pressed && state.just_changed,
			None => false,
		}
	}

	#[allow(dead_code)]
	pub fn is_just_released(&self, code: KeyCode) -> bool {
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

	#[allow(dead_code)]
	pub fn wheel_delta(&self) -> f32 {
		self.wheel_delta
	}
	pub fn update(&mut self) {
		if self.active {
			for (_key, state) in self.keys_states.iter_mut() {
				state.just_changed = false;
			}
			self.mouse_state.left_click.just_changed = false;
			self.mouse_state.right_click.just_changed = false;
			self.mouse_state.rel = Vec2::ZERO;
			self.wheel_delta = 0.0;
			self.active = false;
		}
	}

	pub fn store_key(&mut self, input: KeyboardInput) {
		self.active = true;
		if let Some(v_keycode) = input.virtual_keycode {
			match self.keys_states.get_mut(&v_keycode) {
				Some(state) => {
					state.update(input.state);
				}
				None => {
					let new_state = KeyState::new(input.state == ElementState::Pressed);
					self.keys_states.insert(v_keycode, new_state);
				}
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

	pub fn store_mouse_wheel(&mut self, delta: (f32, f32)) {
		self.active = true;
		self.wheel_delta += delta.1;
	}
}
