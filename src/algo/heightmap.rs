use crate::engine::Entity;

use glam::Vec3;
use std::any::Any;

#[derive(Debug)]
pub struct HeightMap {
	points: Vec<f32>,
	dim: usize,
	step: f32,
}

impl HeightMap {
	pub fn new(points: Vec<f32>, dim: usize, step: f32) -> Self {
		HeightMap { points, dim, step }
	}

	pub fn gen_mesh_vertices(&self) -> Vec<Vec3> {
		let mut vertices = Vec::new();
		let one = 1.0 * self.step;
		for i in 0..self.dim {
			let x = i as f32 * self.step;
			for j in 0..self.dim {
				let y = j as f32 * self.step;
				if i + 1 < self.dim && j + 1 < self.dim {
					let top_left = Vec3::new(x, self.points[i + j * self.dim], y);
					let top_right = Vec3::new(x + one, self.points[i + 1 + j * self.dim], y);
					let bot_left = Vec3::new(x, self.points[i + (j + 1) * self.dim], y + one);
					let bot_right =
						Vec3::new(x + one, self.points[i + 1 + (j + 1) * self.dim], y + one);

					if (top_left.y - bot_right.y).abs() > (top_right.y - bot_left.y).abs() {
						// first triangle
						vertices.push(top_left);
						vertices.push(top_right);
						vertices.push(bot_right);
						// second triangle
						vertices.push(top_left);
						vertices.push(bot_left);
						vertices.push(bot_right);
					} else {
						// first triangle
						vertices.push(top_right);
						vertices.push(top_left);
						vertices.push(bot_left);
						// second triangle
						vertices.push(top_right);
						vertices.push(bot_right);
						vertices.push(bot_left);
					}
				}
			}
		}
		return vertices;
	}
}

impl Entity for HeightMap {
	fn as_any(&self) -> &dyn Any {
		self
	}
}
