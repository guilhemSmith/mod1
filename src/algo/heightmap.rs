use crate::engine::Entity;

use glam::{Vec2, Vec3};
use std::any::Any;

const WEIGHT_DIST: f32 = 5.0;

#[derive(Debug)]
pub struct HeightMap {
	points: Vec<f32>,
}

impl HeightMap {
	pub fn new(poi: Vec<Vec3>) -> Self {
		let map = HeightMap::poi_to_map(poi);
		HeightMap { points: map }
	}

	pub fn height_points(&self) -> &Vec<f32> {
		&self.points
	}

	fn add_border_zero(poi: &mut Vec<Vec3>) {
		let max_val = (super::DIM - 1) as f32;
		poi.push(Vec3::new(0.0, 0.0, 0.0));
		poi.push(Vec3::new(0.0, max_val, 0.0));
		poi.push(Vec3::new(max_val, 0.0, 0.0));
		poi.push(Vec3::new(max_val, max_val, 0.0));
		for i in 1..(super::DIM - 1) {
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
		let mut map = vec![0.0; super::DIM * super::DIM];
		for i in 0..super::DIM {
			let x = i as f32;
			for j in 0..super::DIM {
				let y = j as f32;
				if let Some(point) = poi.iter().find(|pt| pt.x == x && pt.y == y) {
					map[i + j * super::DIM] = point.z
				} else {
					map[i + j * super::DIM] = HeightMap::idw(Vec2::new(x, y), &poi);
				}
			}
		}
		return map;
	}
}

impl Entity for HeightMap {
	fn as_any(&self) -> &dyn Any {
		self
	}
}
