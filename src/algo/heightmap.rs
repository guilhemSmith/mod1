use super::{Map, DIM};
use crate::engine::{Entity, EntityStore, Mesh};

use glam::{Vec2, Vec3};
use std::any::Any;
use std::fs::File;
use std::io::{BufRead, BufReader};

const WEIGHT_DIST: f32 = 5.0;
const MAP_SIZE: usize = DIM * DIM;
const MAP_SCALE: f32 = 1.0;

#[derive(Debug)]
pub struct HeightMap {
	points: Map<MAP_SIZE>,
}

impl HeightMap {
	pub fn new(filename: &str) -> Result<Self, String> {
		let mut poi: Vec<Vec3> = Vec::new();
		let file = File::open(filename).map_err(|err| format!("Failed to read file: {}", err))?;
		let reader = BufReader::new(file);
		for (index_l, line) in reader.lines().enumerate() {
			let line = line.map_err(|err| format!("Failed to read file: {}", err))?;
			let mut point: [f32; 3] = [0.0; 3];
			let mut count = 0;
			for (index_v, value) in line.split_ascii_whitespace().enumerate() {
				count += 1;
				if count > 3 {
					return Err(format!("[line: {}] Too many values", index_l + 1));
				}
				point[index_v] = value
					.parse()
					.map_err(|err| {
						format!(
							"[line: {}, pos: {}] Invalid value '{}' ({})",
							index_l + 1,
							index_v + 1,
							value,
							err
						)
					})
					.and_then(|number| {
						if index_v < 2 {
							match number {
								n if n < 100.0 && n > 0.0 => Ok(n),
								n => Err(format!(
								"[line: {}, pos: {}] Invalid coordinate number '{}' (not between 0 and 99)",
								index_l + 1, index_v + 1, n
							)),
							}
						} else {
							match number {
								n if n < 50.0 && n > -50.0 => Ok(n),
								n => Err(format!(
								"[line: {}, pos: {}] Invalid height number '{}' (not between -50 and 50)",
								index_l + 1, index_v + 1, n
							)),
							}
						}
					})?;
			}
			if count < 3 {
				return Err(format!("[line: {}] Not enough values", index_l + 1));
			}
			poi.push(Vec3::from(point));
		}
		let scaled = poi.into_iter().map(|pt| pt * MAP_SCALE).collect();
		let map = HeightMap::poi_to_map(scaled);
		Ok(HeightMap { points: map })
	}

	pub fn height_points(&self) -> &Map<MAP_SIZE> {
		&self.points
	}

	pub fn border_wall(&self) -> Vec<Vec3> {
		let dim = (DIM - 1) as f32;
		vec![
			Vec3::new(0.0, 0.0, -50.0),
			Vec3::new(0.0, 0.0, 0.0),
			Vec3::new(dim, 0.0, -50.0),
			Vec3::new(dim, 0.0, 0.0),
			Vec3::new(dim, dim, -50.0),
			Vec3::new(dim, dim, 0.0),
			Vec3::new(0.0, dim, -50.0),
			Vec3::new(0.0, dim, 0.0),
			Vec3::new(0.0, 0.0, -50.0),
			Vec3::new(0.0, 0.0, 0.0),
		]
	}

	fn add_border_zero(poi: &mut Vec<Vec3>) {
		let max_val = (DIM - 1) as f32;
		poi.push(Vec3::new(0.0, 0.0, 0.0));
		poi.push(Vec3::new(0.0, max_val, 0.0));
		poi.push(Vec3::new(max_val, 0.0, 0.0));
		poi.push(Vec3::new(max_val, max_val, 0.0));
		for i in 1..(DIM - 1) {
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

	fn poi_to_map(mut poi: Vec<Vec3>) -> [f32; MAP_SIZE] {
		HeightMap::add_border_zero(&mut poi);
		let mut map = [0.0; MAP_SIZE];
		for i in 0..DIM {
			let x = i as f32;
			for j in 0..DIM {
				let y = j as f32;
				if let Some(point) = poi.iter().find(|pt| pt.x == x && pt.y == y) {
					map[i + j * DIM] = point.z
				} else {
					map[i + j * DIM] = HeightMap::idw(Vec2::new(x, y), &poi);
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

	fn as_any_mut(&mut self) -> &mut dyn Any {
		self
	}

	fn start(&mut self, store: &EntityStore) {
		let terrain_vert = Mesh::heights_gen_vertices(DIM, &Vec::from(self.points));
		let terrain_mesh = Box::new(Mesh::new("terrain", &terrain_vert, DIM, true, true));
		let border_vert = Mesh::wall_gen_vertices(&self.border_wall());
		let border_mesh = Box::new(Mesh::new("border", &border_vert, DIM, true, true));
		store.to_new_queue(terrain_mesh);
		store.to_new_queue(border_mesh);
	}
}
