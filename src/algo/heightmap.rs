use crate::engine::Entity;

use glam::{Vec2, Vec3};
use std::any::Any;

const MAP_DIM: usize = 100;
const WEIGHT_DIST: f32 = 5.0;

#[derive(Debug)]
pub struct HeightMap {
	points: Vec<f32>,
	scale: f32,
}

impl HeightMap {
	pub fn new(poi: Vec<Vec3>, scale: f32) -> Self {
		let map = HeightMap::poi_to_map(poi);
		HeightMap { points: map, scale }
	}

	fn add_border_zero(poi: &mut Vec<Vec3>) {
		let max_val = (MAP_DIM - 1) as f32;
		poi.push(Vec3::new(0.0, 0.0, 0.0));
		poi.push(Vec3::new(0.0, max_val, 0.0));
		poi.push(Vec3::new(max_val, 0.0, 0.0));
		poi.push(Vec3::new(max_val, max_val, 0.0));
		for i in 1..(MAP_DIM - 1) {
			let variant = i as f32;
			poi.push(Vec3::new(variant, 0.0, 0.0));
			poi.push(Vec3::new(0.0, variant, 0.0));
			poi.push(Vec3::new(variant, max_val, 0.0));
			poi.push(Vec3::new(max_val, variant, 0.0));
		}
	}

	fn idw(coord: Vec2, poi: &Vec<Vec3>) -> f32 {
		let mut top = 0.0;
		let mut bot = 0.0;
		for i in poi.iter() {
			let k_coord = Vec2::new(i.x, i.y);
			let k_value = i.z;
			let inv_dist = (1.0 / (coord - k_coord).length()).powf(WEIGHT_DIST);
			top += inv_dist * k_value;
			bot += inv_dist;
		}
		return top / bot;
	}

	fn poi_to_map(mut poi: Vec<Vec3>) -> Vec<f32> {
		HeightMap::add_border_zero(&mut poi);
		let mut map = vec![0.0; MAP_DIM * MAP_DIM];
		for i in 0..MAP_DIM {
			let x = i as f32;
			for j in 0..MAP_DIM {
				let y = j as f32;
				if let Some(point) = poi.iter().find(|pt| pt.x == x && pt.y == y) {
					map[i + j * MAP_DIM] = point.z
				} else {
					map[i + j * MAP_DIM] = HeightMap::idw(Vec2::new(x, y), &poi);
				}
			}
		}
		return map;
	}

	pub fn gen_mesh_vertices(&self) -> Vec<Vec3> {
		let mut vertices = Vec::new();
		let one = 1.0 * self.scale;
		for i in 0..MAP_DIM {
			let x = i as f32 * self.scale;
			for j in 0..MAP_DIM {
				let y = j as f32 * self.scale;
				if i + 1 < MAP_DIM && j + 1 < MAP_DIM {
					let top_left = Vec3::new(x, self.points[i + j * MAP_DIM] * self.scale, y);
					let top_right =
						Vec3::new(x + one, self.points[i + 1 + j * MAP_DIM] * self.scale, y);
					let bot_left =
						Vec3::new(x, self.points[i + (j + 1) * MAP_DIM] * self.scale, y + one);
					let bot_right = Vec3::new(
						x + one,
						self.points[i + 1 + (j + 1) * MAP_DIM] * self.scale,
						y + one,
					);

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
