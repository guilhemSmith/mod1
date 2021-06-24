use super::{Water, DIM, G};
use crate::engine::{Entity, EntityStore, Inputs, KeyCode, MeshPoints};

use glam::Vec3;
use std::any::Any;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Rain {
	mesh_id: u128,
	water_id: u128,
	droplets: Vec<Vec3>,
}

impl Rain {
	const MAX_COUNT: usize = 500;

	pub fn new(store: &EntityStore, water_id: u128) -> Self {
		let rain_vert =
			MeshPoints::points_vertices(&vec![Vec3::new(50.0, 50.0, -100.0); Rain::MAX_COUNT]);
		let rain_mesh = Box::new(MeshPoints::new("rain", &rain_vert, DIM, false, false));
		let mesh_id = store.to_new_queue(rain_mesh);
		Rain {
			mesh_id,
			water_id,
			droplets: Vec::new(),
		}
	}

	fn handle_inputs(&mut self, inputs: &Inputs, store: &EntityStore) {
		if inputs.is_pressed(KeyCode::R) {
			if let Some(ent_water) = store.get(self.water_id) {
				if let Some(water) = ent_water.as_any().downcast_ref::<Water>() {
					let strength = 3 + 2 * water.input_str();
					for _ in 0..strength {
						if self.droplets.len() < Rain::MAX_COUNT
							&& water.get_avg_depth() < Water::MAX_HEIGHT
						{
							let droplet = Vec3::new(
								(rand::random::<usize>() % DIM) as f32,
								(rand::random::<usize>() % DIM) as f32,
								75.0,
							);
							self.droplets.push(droplet);
						} else {
							break;
						}
					}
				}
			}
		}
	}

	fn update_mesh(&self, store: &EntityStore) {
		if let Some(ent_mesh) = store.get(self.mesh_id) {
			if let Some(mesh) = ent_mesh.as_any().downcast_ref::<MeshPoints>() {
				mesh.update_vertices(|data| {
					for (id, droplet) in self.droplets.iter().enumerate() {
						data[id * 3 + 0] = droplet.x;
						data[id * 3 + 1] = droplet.z;
						data[id * 3 + 2] = droplet.y;
					}
					for id in self.droplets.len()..Rain::MAX_COUNT {
						data[id * 3 + 0] = 50.0;
						data[id * 3 + 1] = -100.0;
						data[id * 3 + 2] = 50.0;
					}
				});
			}
		}
	}

	fn update_droplets(&mut self, delta: f32, store: &EntityStore) {
		if let Some(mut ent_water) = store.get_mut(self.water_id) {
			if let Some(water) = ent_water.as_any_mut().downcast_mut::<Water>() {
				let mut dead_droplet = HashSet::new();
				for id in 0..self.droplets.len() {
					let droplet = &mut self.droplets[id];
					droplet.z -= delta * G * 10.0;
					match water.get_height(store, droplet.x as usize, droplet.y as usize) {
						Some(height) if height < droplet.z => continue,
						Some(_) => water.add_depth(droplet.x as usize, droplet.y as usize, 1.0),
						None => {}
					}
					dead_droplet.insert((droplet.x as u32, droplet.y as u32, droplet.z as u32));
				}
				self.droplets
					.retain(|d| !dead_droplet.contains(&(d.x as u32, d.y as u32, d.z as u32)))
			}
		}
	}
}

impl Entity for Rain {
	fn as_any(&self) -> &dyn Any {
		self
	}

	fn as_any_mut(&mut self) -> &mut dyn Any {
		self
	}

	fn update(&mut self, delta: f32, inputs: &Inputs, store: &EntityStore) {
		self.update_droplets(delta, store);
		self.handle_inputs(inputs, store);
		self.update_mesh(store);
	}
}
