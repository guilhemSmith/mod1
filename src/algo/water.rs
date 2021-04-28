use super::{HeightMap, Map, DIM};
use crate::engine::{Entity, EntityStore, Inputs, Mesh};

use std::any::Any;

const D_MAP_SIZE: usize = DIM * DIM;
const P_MAP_SIZE: usize = (DIM - 1) * DIM;

const G: f32 = 9.81;
const GRID_STEP: f32 = 1.0;

#[derive(Debug)]
pub struct Water {
	mesh_id: u128,
	terrain_id: u128,
	depths: Map<D_MAP_SIZE>,
	pipes_y: Map<P_MAP_SIZE>,
	pipes_x: Map<P_MAP_SIZE>,
}

impl Water {
	pub fn new(mesh_id: u128, terrain_id: u128) -> Self {
		Water {
			mesh_id,
			terrain_id,
			depths: [0.0; D_MAP_SIZE],
			pipes_y: [0.0; P_MAP_SIZE],
			pipes_x: [0.0; P_MAP_SIZE],
		}
	}

	fn update_pipes_flow(&mut self, delta_time: f32, store: &EntityStore) {
		if let Some(ent_terrain) = store.get(self.terrain_id) {
			if let Some(terrain) = ent_terrain.as_any().downcast_ref::<HeightMap>() {
				for i in 0..(DIM - 1) {
					for j in 0..DIM {
						let height_delta =
							self.depths[i + 1 + j * DIM] + terrain.height_points()[i + 1 + j * DIM]
								- self.depths[i + j * DIM] + terrain.height_points()[i + j * DIM];
						self.pipes_x[i + j * (DIM - 1)] +=
							G / GRID_STEP * height_delta * delta_time;
					}
				}
				for i in 0..DIM {
					for j in 0..(DIM - 1) {
						let height_delta = self.depths[i + (j + 1) * DIM]
							+ terrain.height_points()[i + (j + 1) * DIM]
							- self.depths[i + j * DIM] + terrain.height_points()
							[i + j * DIM];
						self.pipes_y[i + j * (DIM - 1)] +=
							G / GRID_STEP * height_delta * delta_time;
					}
				}
			}
		}
	}

	fn limit_underflow(&mut self, delta_time: f32) {
		for i in 0..DIM {
			for j in 0..DIM {
				let flow_sum_x = match i {
					0 => self.pipes_x[i + j * (DIM - 1)],
					l if l == DIM - 1 => self.pipes_y[i - 1 + j * (DIM - 1)],
					_ => self.pipes_x[i + j * (DIM - 1)] + self.pipes_x[i - 1 + j * (DIM - 1)],
				};
				let flow_sum_y = match j {
					0 => self.pipes_y[i + j * (DIM - 1)],
					l if l == DIM - 1 => self.pipes_y[i + (j - 1) * (DIM - 1)],
					_ => self.pipes_y[i + j * (DIM - 1)] + self.pipes_y[i + (j - 1) * (DIM - 1)],
				};

				let depth_delta = -delta_time * (flow_sum_x * flow_sum_y) / (GRID_STEP * GRID_STEP);
				if depth_delta > 0.0 && depth_delta > self.depths[i + j * DIM] {
					let flow_ratio = self.depths[i + j * DIM] / depth_delta;
					match i {
						0 => self.pipes_x[i + j * (DIM - 1)] *= flow_ratio,
						l if l == DIM - 1 => self.pipes_y[i - 1 + j * (DIM - 1)] *= flow_ratio,
						_ => {
							self.pipes_x[i + j * (DIM - 1)] *= flow_ratio;
							self.pipes_x[i - 1 + j * (DIM - 1)] *= flow_ratio;
						}
					};
					match j {
						0 => self.pipes_y[i + j * (DIM - 1)] *= flow_ratio,
						l if l == DIM - 1 => self.pipes_y[i + (j - 1) * (DIM - 1)] *= flow_ratio,
						_ => {
							self.pipes_y[i + j * (DIM - 1)] *= flow_ratio;
							self.pipes_y[i + (j - 1) * (DIM - 1)] *= flow_ratio;
						}
					};
				}
			}
		}
	}

	fn update_depths(&mut self, delta_time: f32) {
		for i in 0..DIM {
			for j in 0..DIM {
				let flow_sum_x = match i {
					0 => self.pipes_x[i + j * (DIM - 1)],
					l if l == DIM - 1 => self.pipes_y[i - 1 + j * (DIM - 1)],
					_ => self.pipes_x[i + j * (DIM - 1)] + self.pipes_x[i - 1 + j * (DIM - 1)],
				};
				let flow_sum_y = match j {
					0 => self.pipes_y[i + j * (DIM - 1)],
					l if l == DIM - 1 => self.pipes_y[i + (j - 1) * (DIM - 1)],
					_ => self.pipes_y[i + j * (DIM - 1)] + self.pipes_y[i + (j - 1) * (DIM - 1)],
				};

				self.depths[i + j * DIM] +=
					-delta_time * (flow_sum_x * flow_sum_y) / (GRID_STEP * GRID_STEP);
			}
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
								if self.depths[x + y * DIM] > 0.0001 {
									data[i * 3 + 1] = self.depths[x + y * DIM]
										+ terrain.height_points()[x + y * DIM];
								} else {
									data[i * 3 + 1] = terrain.height_points()[x + y * DIM] - 0.1;
								}
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
		if _inputs.is_pressed(Inputs::K_ENTER) {
			self.depths[0] = 20.0;
		}
		self.update_pipes_flow(delta, store);
		self.limit_underflow(delta);
		self.update_depths(delta);
		self.update_mesh(store);
	}
}
