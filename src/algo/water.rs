use super::{HeightMap, Map, DIM};
use crate::engine::{Entity, EntityStore, Inputs, Mesh};

use std::any::Any;

const ZERO_DEPTH: f32 = 0.01;

const D_MAP_SIZE: usize = DIM * DIM;
const P_MAP_SIZE: usize = (DIM - 1) * DIM;

const G: f32 = 9.81;
const GRID_STEP: f32 = 0.1;

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
						let height_delta = (self.depths[i + 1 + j * DIM]
							+ terrain.height_points()[i + 1 + j * DIM])
							- (self.depths[i + j * DIM] + terrain.height_points()[i + j * DIM]);
						self.pipes_x[i + j * (DIM - 1)] +=
							0.1 * G / GRID_STEP * height_delta * delta_time;
						self.pipes_x[i + j * (DIM - 1)] *= 1.0 - delta_time * GRID_STEP * 2.0;
					}
				}
				for i in 0..DIM {
					for j in 0..(DIM - 1) {
						let height_delta = self.depths[i + (j + 1) * DIM]
							+ terrain.height_points()[i + (j + 1) * DIM]
							- (self.depths[i + j * DIM] + terrain.height_points()[i + j * DIM]);
						self.pipes_y[i + j * DIM] +=
							0.1 * G / GRID_STEP * height_delta * delta_time;
						self.pipes_y[i + j * DIM] *= 1.0 - delta_time * GRID_STEP * 2.0;
					}
				}
			}
		}
	}

	fn capped_flow(&mut self, delta_time: f32) {
		for i in 0..DIM {
			for j in 0..DIM {
				let mut flow_in = 0.0;
				let mut flow_out = 0.0;
				let mut split_flow = |flow: f32| {
					let flow = flow;
					if flow > ZERO_DEPTH {
						flow_out += flow;
					} else if flow < -ZERO_DEPTH {
						flow_in -= flow;
					}
				};
				if i < DIM - 1 {
					split_flow(-self.pipes_x[i + j * (DIM - 1)]);
				}
				if i > 0 {
					split_flow(self.pipes_x[i - 1 + j * (DIM - 1)]);
				}
				if j < DIM - 1 {
					split_flow(-self.pipes_y[i + j * DIM]);
				}
				if j > 0 {
					split_flow(self.pipes_y[i + (j - 1) * DIM]);
				}
				let flow_sum = flow_in - flow_out;

				let delta_depth = delta_time * flow_sum / (GRID_STEP * GRID_STEP);
				if delta_depth < 0.0 && self.depths[i + j * DIM] + delta_depth < 0.0 {
					let to_refund = -(delta_depth + self.depths[i + j * DIM]);
					let ratio = 1.0 - (to_refund / -delta_depth);
					if i < DIM - 1 && -self.pipes_x[i + j * (DIM - 1)] > 0.0 {
						self.pipes_x[i + j * (DIM - 1)] *= ratio;
					}
					if i > 0 && self.pipes_x[i - 1 + j * (DIM - 1)] > 0.0 {
						self.pipes_x[i - 1 + j * (DIM - 1)] *= ratio;
					}
					if j < DIM - 1 && -self.pipes_y[i + j * DIM] > 0.0 {
						self.pipes_y[i + j * DIM] *= ratio;
					}
					if j > 0 && self.pipes_y[i + (j - 1) * DIM] > 0.0 {
						self.pipes_y[i + (j - 1) * DIM] *= ratio;
					}
				}
			}
		}
	}

	fn update_depths(&mut self, delta_time: f32) {
		let mut depth_sum = 0.0;
		for i in 0..DIM {
			for j in 0..DIM {
				let mut flow_in = 0.0;
				let mut flow_out = 0.0;
				let mut split_flow = |flow: f32| {
					let flow = flow;
					if flow > ZERO_DEPTH {
						flow_out += flow;
					} else if flow < -ZERO_DEPTH {
						flow_in -= flow;
					}
				};
				if i < DIM - 1 {
					split_flow(-self.pipes_x[i + j * (DIM - 1)]);
				}
				if i > 0 {
					split_flow(self.pipes_x[i - 1 + j * (DIM - 1)]);
				}
				if j < DIM - 1 {
					split_flow(-self.pipes_y[i + j * DIM]);
				}
				if j > 0 {
					split_flow(self.pipes_y[i + (j - 1) * DIM]);
				}
				let flow_sum = flow_in - flow_out;

				let delta_depth = delta_time * flow_sum / (GRID_STEP * GRID_STEP);
				self.depths[i + j * DIM] += delta_depth;
			}
		}
		for i in 0..DIM {
			for j in 0..DIM {
				depth_sum += self.depths[i + j * DIM];
			}
		}
		println!("depth sum: {}", depth_sum);
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
								if self.depths[x + y * DIM] > ZERO_DEPTH {
									data[i * 3 + 1] = self.depths[x + y * DIM]
										+ terrain.height_points()[x + y * DIM];
								} else {
									if (x > 0 && self.depths[x - 1 + y * DIM] > ZERO_DEPTH)
										|| (x < DIM - 1
											&& self.depths[x + 1 + y * DIM] > ZERO_DEPTH)
										|| (y > 0 && self.depths[x + (y - 1) * DIM] > ZERO_DEPTH)
										|| (y < DIM - 1
											&& self.depths[x + (y + 1) * DIM] > ZERO_DEPTH)
									{
										data[i * 3 + 1] =
											terrain.height_points()[x + y * DIM] + ZERO_DEPTH;
									} else {
										data[i * 3 + 1] =
											terrain.height_points()[x + y * DIM] - ZERO_DEPTH;
									}
									// data[i * 3 + 1] = terrain.height_points()[x + y * DIM] - 0.1;
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
			for i in (DIM / 4)..(3 * DIM / 4) {
				self.depths[i] += 1.0;
			}
		}
		self.update_pipes_flow(delta, store);
		self.capped_flow(delta);
		self.update_depths(delta);
		self.update_mesh(store);
	}
}
