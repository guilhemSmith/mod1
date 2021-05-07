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
				let flow_acceleration =
					|depth: f32, depth_next: f32, terrain: f32, terrain_next: f32| {
						let height_delta = (depth_next + terrain_next) - (depth + terrain);
						let upwind_depth = f32::min(f32::max(depth, depth_next), 5.0);
						return upwind_depth * G * height_delta * delta_time;
					};
				let flow_decceleration = f32::min(1.0 - delta_time * 1.5, 1.0);

				for i in 0..(DIM - 1) {
					for j in 0..DIM {
						if self.depths[i + 1 + j * DIM] > ZERO_DEPTH
							|| self.depths[i + j * DIM] > ZERO_DEPTH
						{
							self.pipes_x[i + j * (DIM - 1)] += flow_acceleration(
								self.depths[i + j * DIM],
								self.depths[i + 1 + j * DIM],
								terrain.height_points()[i + j * DIM],
								terrain.height_points()[i + 1 + j * DIM],
							);
							self.pipes_x[i + j * (DIM - 1)] *= flow_decceleration;
						} else {
							self.pipes_x[i + j * (DIM - 1)] = 0.0;
						}
					}
				}
				for i in 0..DIM {
					for j in 0..(DIM - 1) {
						if self.depths[i + (j + 1) * DIM] > ZERO_DEPTH
							|| self.depths[i + j * DIM] > ZERO_DEPTH
						{
							self.pipes_y[i + j * DIM] += flow_acceleration(
								self.depths[i + j * DIM],
								self.depths[i + (j + 1) * DIM],
								terrain.height_points()[i + j * DIM],
								terrain.height_points()[i + (j + 1) * DIM],
							);
							self.pipes_y[i + j * DIM] *= flow_decceleration;
						} else {
							self.pipes_y[i + j * DIM] = 0.0;
						}
					}
				}
			}
		}
	}

	fn cap_flow(
		&mut self,
		i: usize,
		j: usize,
		to_check: &mut Vec<(usize, usize)>,
		delta_time: f32,
	) {
		let (flow_in, flow_out) = self.compute_flows(i, j);
		let flow_sum = flow_in - flow_out;
		let delta_depth = delta_time * flow_sum / (GRID_STEP * GRID_STEP);
		if self.depths[i + j * DIM] + delta_depth < 0.0 {
			let capped_out =
				flow_in + (self.depths[i + j * DIM] * (GRID_STEP * GRID_STEP)) / delta_time;
			let mut ratio = capped_out / flow_out;
			if ratio.abs() >= 1.0 {
				ratio = 0.0;
			}
			if i < DIM - 1 && -self.pipes_x[i + j * (DIM - 1)] > 0.0 {
				self.pipes_x[i + j * (DIM - 1)] *= ratio;
				if !to_check.contains(&(i + 1, j)) {
					to_check.push((i + 1, j));
				}
			}
			if i > 0 && self.pipes_x[i - 1 + j * (DIM - 1)] > 0.0 {
				self.pipes_x[i - 1 + j * (DIM - 1)] *= ratio;
				if !to_check.contains(&(i - 1, j)) {
					to_check.push((i - 1, j));
				}
			}
			if j < DIM - 1 && -self.pipes_y[i + j * DIM] > 0.0 {
				self.pipes_y[i + j * DIM] *= ratio;
				if !to_check.contains(&(i, j + 1)) {
					to_check.push((i, j + 1));
				}
			}
			if j > 0 && self.pipes_y[i + (j - 1) * DIM] > 0.0 {
				self.pipes_y[i + (j - 1) * DIM] *= ratio;
				if !to_check.contains(&(i, j - 1)) {
					to_check.push((i, j - 1));
				}
			}
		}
	}

	fn limit_flows(&mut self, delta_time: f32) {
		let mut to_check = Vec::new();
		for i in 0..DIM {
			for j in 0..DIM {
				self.cap_flow(i, j, &mut to_check, delta_time);
			}
		}
		while let Some((i, j)) = to_check.pop() {
			self.cap_flow(i, j, &mut to_check, delta_time);
		}
	}

	fn compute_flows(&self, i: usize, j: usize) -> (f32, f32) {
		let mut flow_in = 0.0;
		let mut flow_out = 0.0;
		let mut split_flow = |flow: f32| {
			let flow = flow;
			if flow > 0.0 {
				flow_out += flow;
			} else if flow < 0.0 {
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
		return (flow_in, flow_out);
	}

	fn update_depths(&mut self, delta_time: f32) {
		for i in 0..DIM {
			for j in 0..DIM {
				let (flow_in, flow_out) = self.compute_flows(i, j);
				let flow_sum = flow_in - flow_out;

				let delta_depth = delta_time * flow_sum / (GRID_STEP * GRID_STEP);
				self.depths[i + j * DIM] += delta_depth;
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
								if self.depths[x + y * DIM] > ZERO_DEPTH {
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

	fn update(&mut self, delta: f32, inputs: &Inputs, store: &EntityStore) {
		if inputs.is_pressed(Inputs::K_W) {
			for i in 0..DIM {
				self.depths[i] += 1.0;
			}
		}
		if let Some(ent_terrain) = store.get(self.terrain_id) {
			if let Some(terrain) = ent_terrain.as_any().downcast_ref::<HeightMap>() {
				if inputs.is_pressed(Inputs::K_T) {
					for i in 0..DIM {
						for j in 0..DIM {
							if terrain.height_points()[i + j * DIM] <= ZERO_DEPTH {
								self.depths[i + j * DIM] += 0.1;
							}
						}
					}
				}

				if inputs.is_pressed(Inputs::K_D) {
					for i in 0..DIM {
						for j in 0..DIM {
							if terrain.height_points()[i + j * DIM] <= ZERO_DEPTH {
								self.depths[i + j * DIM] -= 0.1;
								if self.depths[i + j * DIM] <= ZERO_DEPTH {
									self.depths[i + j * DIM] = 0.0;
									if i < DIM - 1 && self.pipes_x[i + j * (DIM - 1)] < 0.0 {
										self.pipes_x[i + j * (DIM - 1)] = 0.0;
									}
									if i > 0 && self.pipes_x[i - 1 + j * (DIM - 1)] > 0.0 {
										self.pipes_x[i - 1 + j * (DIM - 1)] = 0.0;
									}
									if j < DIM - 1 && self.pipes_y[i + j * DIM] < 0.0 {
										self.pipes_y[i + j * DIM] = 0.0;
									}
									if j > 0 && self.pipes_y[i + (j - 1) * DIM] > 0.0 {
										self.pipes_y[i + (j - 1) * DIM] = 0.0;
									}
								}
							}
						}
					}
				}
			}
		}
		self.update_pipes_flow(delta, store);
		self.limit_flows(delta);
		self.update_depths(delta);
		self.update_mesh(store);
	}
}
