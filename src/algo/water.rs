use crate::engine::{Entity, EntityStore, Inputs, Mesh};

use std::any::Any;

#[derive(Debug)]
pub struct Water {
	mesh_id: u128,
	points: Vec<f32>,
}

impl Water {
	pub fn new(mesh_id: u128) -> Self {
		let points = vec![0.0; super::DIM * super::DIM];

		Water { mesh_id, points }
	}
}

impl Entity for Water {
	fn as_any(&self) -> &dyn Any {
		self
	}

	fn update(&mut self, delta: f32, _inputs: &Inputs, store: &EntityStore) {
		for pt in self.points.iter_mut() {
			*pt = *pt + delta;
		}
		if let Some(entity) = store.get_mut(self.mesh_id) {
			if let Some(mesh) = entity.as_any().downcast_ref::<Mesh>() {
				mesh.update_vertices(|data| {
					for i in 0..(data.len() / 3) as usize {
						let x = data[i * 3] as usize;
						let y = data[i * 3 + 2] as usize;
						data[i * 3 + 1] = self.points[x + y * super::DIM];
					}
				});
			}
		}
	}
}
