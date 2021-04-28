use super::{HeightMap, Map, DIM};
use crate::engine::{Entity, EntityStore, Inputs, Mesh};

use std::any::Any;

const D_MAP_SIZE: usize = DIM * DIM;
const P_MAP_SIZE: usize = (DIM - 1) * DIM;

#[derive(Debug)]
pub struct Water {
	mesh_id: u128,
	terrain_id: u128,
	depths: Map<D_MAP_SIZE>,
	pipes_v: Map<P_MAP_SIZE>,
	pipes_h: Map<P_MAP_SIZE>,
}

impl Water {
	pub fn new(mesh_id: u128, terrain_id: u128) -> Self {
		Water {
			mesh_id,
			terrain_id,
			depths: [0.0; D_MAP_SIZE],
			pipes_v: [0.0; P_MAP_SIZE],
			pipes_h: [0.0; P_MAP_SIZE],
		}
	}

	fn update_mesh(&self, store: &EntityStore) {
		if let Some(ent_terrain) = store.get(self.terrain_id) {
			if let Some(terrain) = ent_terrain.as_any().downcast_ref::<HeightMap>() {
				if let Some(ent_mesh) = store.get_mut(self.mesh_id) {
					if let Some(mesh) = ent_mesh.as_any().downcast_ref::<Mesh>() {
						mesh.update_vertices(|data| {
							for i in 0..(data.len() / 3) as usize {
								let x = data[i * 3] as usize;
								let y = data[i * 3 + 2] as usize;
								data[i * 3 + 1] =
									self.depths[x + y * DIM] + terrain.height_points()[x + y * DIM];
							}
						});
					}
				}
			}
		}
	}
}

impl Entity for Water {
	fn as_any(&self) -> &dyn Any {
		self
	}

	fn update(&mut self, delta: f32, _inputs: &Inputs, store: &EntityStore) {
		for pt in self.depths.iter_mut() {
			*pt = *pt + delta;
		}
		self.update_mesh(store);
	}
}
