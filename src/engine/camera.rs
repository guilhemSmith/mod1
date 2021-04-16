use glam::{Mat4, Vec3};
use std::any::Any;

const NEAR: f32 = 0.1;
const FAR: f32 = 100.0;
const CENTER: Vec3 = Vec3::ZERO;
const UP: Vec3 = Vec3::Y;

#[derive(Debug)]
pub struct Camera {
	fov: f32,
	ratio: f32,
	yaw: f32,
	pitch: f32,
	dist: f32,
	speed: f32,
}

impl Camera {
	pub fn new() -> Self {
		Camera {
			fov: f32::to_radians(80.0),
			ratio: 16.0 / 9.0,
			yaw: 0.0,
			pitch: f32::to_radians(45.0),
			dist: 2.0,
			speed: 50.0,
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
}

impl super::Entity for Camera {
	fn update(&mut self, delta: f32, inputs: &super::Inputs) {
		let axis: f32 = if inputs.is_pressed(105) { 1.0 } else { 0.0 }
			+ if inputs.is_pressed(106) { -1.0 } else { 0.0 };
		self.yaw -= axis.to_radians() * self.speed * delta;

		let axis: f32 = if inputs.is_pressed(103) { 1.0 } else { 0.0 }
			+ if inputs.is_pressed(108) { -1.0 } else { 0.0 };
		self.pitch += axis.to_radians() * self.speed * delta;
		self.pitch = self
			.pitch
			.clamp(f32::to_radians(-80.0), f32::to_radians(80.0));
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}
