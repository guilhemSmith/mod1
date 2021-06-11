use super::{ClickButton, Inputs, KeyCode};
use glam::{Mat4, Vec3};
use std::any::Any;

const NEAR: f32 = 0.1;
const FAR: f32 = 300.0;
const CENTER: Vec3 = Vec3::ZERO;
const UP: Vec3 = Vec3::Y;
const DIST_MIN: f32 = 75.0;
const DIST_MAX: f32 = 100.0;
const DIST_SPEED: f32 = 0.5;

#[derive(Debug)]
pub struct Camera {
	fov: f32,
	ratio: f32,
	yaw: f32,
	pitch: f32,
	dist: f32,
	speed: f32,
	zoom_coef: f32,
	mode: PolygonMode,
}

#[derive(Debug)]
pub enum PolygonMode {
	Point,
	Line,
	Face,
}

impl Camera {
	pub fn new(zoom_natural: bool, mode: Option<PolygonMode>) -> Self {
		Camera {
			fov: f32::to_radians(80.0),
			ratio: 16.0 / 9.0,
			yaw: 0.0,
			pitch: f32::to_radians(45.0),
			dist: 75.0,
			speed: 20.0,
			zoom_coef: DIST_SPEED * if zoom_natural { -1.0 } else { 1.0 },
			mode: mode.unwrap_or(PolygonMode::Face),
		}
	}

	#[allow(dead_code)]
	pub fn perspective(&self) -> Mat4 {
		Mat4::perspective_rh_gl(self.fov, self.ratio, NEAR, FAR)
	}

	#[allow(dead_code)]
	pub fn orthographic(&self) -> Mat4 {
		Mat4::orthographic_rh_gl(-8.0, 8.0, -4.5, 4.5, NEAR, FAR)
	}

	#[allow(dead_code)]
	pub fn view(&self) -> Mat4 {
		let cam_pos = glam::Vec3::new(
			self.yaw.cos() * self.pitch.cos(),
			self.pitch.sin(),
			self.yaw.sin() * self.pitch.cos(),
		)
		.normalize() * self.dist;
		Mat4::look_at_rh(cam_pos, CENTER, UP)
	}

	#[allow(dead_code)]
	pub fn pos(&self) -> Vec3 {
		glam::Vec3::new(
			self.yaw.cos() * self.pitch.cos(),
			self.pitch.sin(),
			self.yaw.sin() * self.pitch.cos(),
		)
		.normalize() * self.dist
	}

	fn change_render_mode(&mut self) {
		match self.mode {
			PolygonMode::Point => self.mode = PolygonMode::Line,
			PolygonMode::Line => self.mode = PolygonMode::Face,
			PolygonMode::Face => self.mode = PolygonMode::Point,
		};
		self.set_render_mode();
	}

	fn set_render_mode(&self) {
		unsafe {
			gl::PolygonMode(
				gl::FRONT_AND_BACK,
				match self.mode {
					PolygonMode::Point => gl::POINT,
					PolygonMode::Line => gl::LINE,
					PolygonMode::Face => gl::FILL,
				},
			);
		}
	}
}

impl super::Entity for Camera {
	fn start(&mut self, _store: &super::EntityStore) {
		self.set_render_mode();
	}

	fn update(&mut self, delta: f32, inputs: &Inputs, _store: &super::EntityStore) {
		if inputs.is_click_pressed(ClickButton::Left) {
			let axis = inputs.mouse_rel();
			self.yaw += axis.x.to_radians() * self.speed * delta;
			self.pitch += axis.y.to_radians() * self.speed * delta;
			self.pitch = self
				.pitch
				.clamp(f32::to_radians(-20.0), f32::to_radians(80.0));
		}
		let dist_delta = inputs.wheel_delta() * self.zoom_coef;
		if dist_delta != 0.0 {
			self.dist = (self.dist - dist_delta).clamp(DIST_MIN, DIST_MAX);
		}
		if inputs.is_just_pressed(KeyCode::Space) {
			self.change_render_mode();
		}
	}

	fn as_any(&self) -> &dyn Any {
		self
	}

	fn as_any_mut(&mut self) -> &mut dyn Any {
		self
	}
}
