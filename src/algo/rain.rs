use super::DIM;
use crate::engine::{Entity, EntityStore, Inputs, KeyCode, MeshPoints};

use glam::Vec3;
use std::any::Any;
use std::collections::LinkedList;

#[derive(Debug)]
pub struct Rain {
	mesh_id: u128,
	water_id: u128,
	droplets: LinkedList<Vec3>,
}

impl Rain {
	pub const MAX_COUNT: usize = 500;

	pub fn new(mesh_id: u128, water_id: u128) -> Self {
		Rain {
			mesh_id,
			water_id,
			droplets: LinkedList::new(),
		}
	}

	fn handle_inputs(&mut self, inputs: &Inputs) {
		if self.droplets.len() < Rain::MAX_COUNT && inputs.is_pressed(KeyCode::R) {
			let droplet = Vec3::new(
				(rand::random::<usize>() % DIM) as f32,
				(rand::random::<usize>() % DIM) as f32,
				50.0,
			);
			self.droplets.push_front(droplet);
			println!("{}", self.droplets.len());
		}
	}
}

impl Entity for Rain {
	fn as_any(&self) -> &dyn Any {
		self
	}

	fn update(&mut self, delta: f32, inputs: &Inputs, store: &EntityStore) {
		// self.update_droplets(delta, store);
		self.handle_inputs(inputs);
		// self.update_mesh(store);
	}
}
