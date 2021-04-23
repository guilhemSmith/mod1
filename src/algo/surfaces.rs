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
				if i > 0 && j > 0 {
					vertices.push(Vec3::new(x - one, self.points[i - 1 + j * self.dim], y));
					vertices.push(Vec3::new(x, self.points[i + j * self.dim], y));
					vertices.push(Vec3::new(x, self.points[i + (j - 1) * self.dim], y - one));
				}
				if i + 1 < self.dim && j + 1 < self.dim {
					vertices.push(Vec3::new(x + one, self.points[i + 1 + j * self.dim], y));
					vertices.push(Vec3::new(x, self.points[i + j * self.dim], y));
					vertices.push(Vec3::new(x, self.points[i + (j + 1) * self.dim], y + one));
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
