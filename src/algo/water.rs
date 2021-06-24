use super::{HeightMap, Map, DIM, G};
use crate::engine::{Entity, EntityStore, Inputs, KeyCode, Mesh};

use glam::Vec3;
use noise::{NoiseFn, Perlin, Seedable, Worley};
use std::any::Any;

const ZERO_DEPTH: f32 = 0.01;

const D_MAP_SIZE: usize = DIM * DIM;
const P_MAP_SIZE: usize = (DIM - 1) * DIM;

const GRID_STEP: f32 = 1.0;

#[derive(Debug)]
pub struct Water {
	mesh_id: u128,
	terrain_id: u128,
	border_id: u128,
	depths: Map<D_MAP_SIZE>,
	pipes_y: Map<P_MAP_SIZE>,
	pipes_x: Map<P_MAP_SIZE>,
	avg_depth: f32,
	input_str: u8,
}

impl Water {
	pub const MAX_HEIGHT: f32 = 50.0;

	pub fn new(store: &EntityStore, terrain_id: u128) -> Self {
		let noise = Water::foam_noise();
		let water_vert = Mesh::heights_gen_vertices(DIM, &vec![-0.1; DIM * DIM]);
		let water_mesh = Box::new(Mesh::new(
			"water",
			&water_vert,
			DIM,
			false,
			false,
			Some(&noise),
		));
		let mesh_id = store.to_new_queue(water_mesh);
		let border_vert = Mesh::wall_gen_vertices(&vec![Vec3::ZERO; 800]);
		let border_mesh = Box::new(Mesh::new("water", &border_vert, DIM, false, false, None));
		let border_id = store.to_new_queue(border_mesh);
		Water {
			mesh_id,
			terrain_id,
			border_id,
			depths: [0.0; D_MAP_SIZE],
			pipes_y: [0.0; P_MAP_SIZE],
			pipes_x: [0.0; P_MAP_SIZE],
			avg_depth: 0.0,
			input_str: 1,
		}
	}

	fn interpolate_noise(x: i32, y: i32, scale: f64, noise: &Perlin) -> u8 {
		if x < 974 && y < 974 {
			(((noise.get([x as f64 / scale * 10.0, y as f64 / scale * 10.0]) + 1.0) / 2.0) * 255.0)
				as u8
		} else {
			if y < 974 {
				let last =
					(noise.get([974 as f64 / scale * 10.0, y as f64 / scale * 10.0]) + 1.0) / 2.0;
				let first =
					(noise.get([0 as f64 / scale * 10.0, y as f64 / scale * 10.0]) + 1.0) / 2.0;
				let k = (x - 974) as f64 / 50.0;
				((last * (1.0 - k) + first * k) * 255.0) as u8
			} else if x < 974 {
				let last =
					(noise.get([x as f64 / scale * 10.0, 974 as f64 / scale * 10.0]) + 1.0) / 2.0;
				let first =
					(noise.get([x as f64 / scale * 10.0, 0 as f64 / scale * 10.0]) + 1.0) / 2.0;
				let k = (y - 974) as f64 / 50.0;
				((last * (1.0 - k) + first * k) * 255.0) as u8
			} else {
				let last =
					(noise.get([974 as f64 / scale * 10.0, 974 as f64 / scale * 10.0]) + 1.0) / 2.0;
				let first =
					(noise.get([974 as f64 / scale * 10.0, 0 as f64 / scale * 10.0]) + 1.0) / 2.0;
				let k = (y - 974) as f64 / 50.0;
				let last_interpol = last * (1.0 - k) + first * k;
				let last =
					(noise.get([0 as f64 / scale * 10.0, 974 as f64 / scale * 10.0]) + 1.0) / 2.0;
				let first =
					(noise.get([0 as f64 / scale * 10.0, 0 as f64 / scale * 10.0]) + 1.0) / 2.0;
				let k = (y - 974) as f64 / 50.0;
				let first_interpol = last * (1.0 - k) + first * k;
				let k = (x - 974) as f64 / 50.0;
				((last_interpol * (1.0 - k) + first_interpol * k) * 255.0) as u8
			}
		}
	}

	fn foam_noise() -> [[u8; 3]; 1024 * 1024] {
		let whorley = Worley::default()
			.set_frequency(10.0)
			.set_seed(rand::random::<u32>());
		let perlin_x = Perlin::default().set_seed(rand::random::<u32>());
		let perlin_y = Perlin::default().set_seed(rand::random::<u32>());

		let edge_w = 3;
		let scale = 1024.0;
		let mut edge = [[0; 3]; 1024 * 1024];
		for x in 0..1024 {
			for y in 0..1024 {
				let x_pr = if x >= edge_w {
					x - edge_w
				} else {
					1023 - x - edge_w
				};
				let x_nx = if x <= 1023 - edge_w {
					x + edge_w
				} else {
					x + edge_w - 1023
				};
				let y_pr = if y >= edge_w {
					y - edge_w
				} else {
					1023 - y - edge_w
				};
				let y_nx = if y <= 1023 - edge_w {
					y + edge_w
				} else {
					y + edge_w - 1023
				};
				let val_l = (whorley.get([x_pr as f64 / scale, y as f64 / scale]) + 1.0) / 2.0;
				let val_r = (whorley.get([x_nx as f64 / scale, y as f64 / scale]) + 1.0) / 2.0;
				let val_top = (whorley.get([x as f64 / scale, y_pr as f64 / scale]) + 1.0) / 2.0;
				let val_bot = (whorley.get([x as f64 / scale, y_nx as f64 / scale]) + 1.0) / 2.0;
				let d_x = (val_l - val_r).abs();
				let d_y = (val_top - val_bot).abs();
				let noise_x = Water::interpolate_noise(x, y, scale, &perlin_x);
				let noise_y = Water::interpolate_noise(x, y, scale, &perlin_y);
				if d_x > 0.0 || d_y > 0.0 {
					edge[(x + y * 1024) as usize] = [255, noise_x, noise_y];
				} else {
					edge[(x + y * 1024) as usize] = [0, noise_x, noise_y];
				}
			}
		}
		return edge;
	}

	fn update_pipes_flow(&mut self, delta_time: f32, store: &EntityStore) {
		if let Some(ent_terrain) = store.get(self.terrain_id) {
			if let Some(terrain) = ent_terrain.as_any().downcast_ref::<HeightMap>() {
				let flow_acceleration =
					|depth: f32, depth_next: f32, terrain: f32, terrain_next: f32| {
						let height_delta = (depth_next + terrain_next) - (depth + terrain);
						let upwind_depth = f32::min(f32::max(depth, depth_next), 5.0);
						return 1.0 * upwind_depth * G * height_delta * delta_time;
					};
				let flow_decceleration = f32::min(1.0 - delta_time * 0.05, 1.0);

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
		self.avg_depth = 0.0;
		for i in 0..DIM {
			for j in 0..DIM {
				let (flow_in, flow_out) = self.compute_flows(i, j);
				let flow_sum = flow_in - flow_out;

				let delta_depth = delta_time * flow_sum / (GRID_STEP * GRID_STEP);
				self.depths[i + j * DIM] += delta_depth;
				self.avg_depth += self.depths[i + j * DIM];
			}
		}
		self.avg_depth /= (DIM * DIM) as f32;
	}

	fn update_mesh(&self, store: &EntityStore) {
		if let Some(ent_terrain) = store.get(self.terrain_id) {
			if let Some(terrain) = ent_terrain.as_any().downcast_ref::<HeightMap>() {
				if let Some(ent_mesh) = store.get_mut(self.mesh_id) {
					if let Some(mesh) = ent_mesh.as_any().downcast_ref::<Mesh>() {
						mesh.update_vertices(|data| {
							for i in 0..(data.len() / 6) as usize {
								let x = data[i * 6] as usize;
								let y = data[i * 6 + 2] as usize;
								if self.depths[x + y * DIM] > ZERO_DEPTH {
									data[i * 6 + 1] = self.depths[x + y * DIM]
										+ terrain.height_points()[x + y * DIM];
								} else {
									let check_neighbor = |n_index: usize, sum: &mut f32| {
										if self.depths[n_index] > ZERO_DEPTH
											&& terrain.height_points()[x + y * DIM]
												> terrain.height_points()[n_index]
										{
											*sum += terrain.height_points()[n_index]
												+ self.depths[n_index];
											return 1.0;
										}
										return 0.0;
									};
									let mut n_count = 0.0;
									let mut n_sum = 0.0;
									if x > 0 {
										n_count += check_neighbor(x - 1 + y * DIM, &mut n_sum);
									}

									if x < DIM - 1 {
										n_count += check_neighbor(x + 1 + y * DIM, &mut n_sum);
									}

									if y > 0 {
										n_count += check_neighbor(x + (y - 1) * DIM, &mut n_sum);
										if x > 0 {
											n_count +=
												check_neighbor(x - 1 + (y - 1) * DIM, &mut n_sum);
										}
										if x < DIM - 1 {
											n_count +=
												check_neighbor(x + 1 + (y - 1) * DIM, &mut n_sum);
										}
									}

									if y < DIM - 1 {
										n_count += check_neighbor(x + (y + 1) * DIM, &mut n_sum);
										if x > 0 {
											n_count +=
												check_neighbor(x - 1 + (y + 1) * DIM, &mut n_sum);
										}
										if x < DIM - 1 {
											n_count +=
												check_neighbor(x + 1 + (y + 1) * DIM, &mut n_sum);
										}
									}

									if n_count == 0.0 {
										data[i * 6 + 1] =
											terrain.height_points()[x + y * DIM] - 1.0;
									} else {
										data[i * 6 + 1] = n_sum / n_count;
									}
								}
								let prev_x = if x > 0 && self.depths[x - 1 + y * DIM] > ZERO_DEPTH {
									terrain.height_points()[x - 1 + y * DIM]
										+ self.depths[x - 1 + y * DIM]
								} else {
									terrain.height_points()[x + y * DIM] + self.depths[x + y * DIM]
								};
								let next_x = if x < DIM - 1
									&& self.depths[x + 1 + y * DIM] > ZERO_DEPTH
								{
									terrain.height_points()[x + 1 + y * DIM]
										+ self.depths[x + 1 + y * DIM]
								} else {
									terrain.height_points()[x + y * DIM] + self.depths[x + y * DIM]
								};
								let prev_y = if y > 0 && self.depths[x + (y - 1) * DIM] > ZERO_DEPTH
								{
									terrain.height_points()[x + (y - 1) * DIM]
										+ self.depths[x + (y - 1) * DIM]
								} else {
									terrain.height_points()[x + y * DIM] + self.depths[x + y * DIM]
								};
								let next_y = if y < DIM - 1
									&& self.depths[x + (y + 1) * DIM] > ZERO_DEPTH
								{
									terrain.height_points()[x + (y + 1) * DIM]
										+ self.depths[x + (y + 1) * DIM]
								} else {
									terrain.height_points()[x + y * DIM] + self.depths[x + y * DIM]
								};
								let normal = Mesh::normal(prev_x, next_x, prev_y, next_y);
								data[i * 6 + 3] = normal.x;
								data[i * 6 + 4] = normal.z;
								data[i * 6 + 5] = normal.y;
							}
						});
					}
				}
			}
		}
	}

	fn update_border_mesh(&self, store: &EntityStore) {
		if let Some(ent_terrain) = store.get(self.terrain_id) {
			if let Some(terrain) = ent_terrain.as_any().downcast_ref::<HeightMap>() {
				let mut bord_points = Vec::new();
				for x in 0..(DIM - 1) {
					bord_points.push(Vec3::new(x as f32, 0.0, 0.0));
					bord_points.push(Vec3::new(
						x as f32,
						0.0,
						self.depths[x] + terrain.height_points()[x],
					));
				}
				for y in 0..(DIM - 1) {
					bord_points.push(Vec3::new((DIM - 1) as f32, y as f32, 0.0));
					bord_points.push(Vec3::new(
						(DIM - 1) as f32,
						y as f32,
						self.depths[DIM - 1 + y * DIM] + terrain.height_points()[DIM - 1 + y * DIM],
					));
				}

				for x in (0..(DIM)).rev() {
					bord_points.push(Vec3::new(x as f32, (DIM - 1) as f32, 0.0));
					bord_points.push(Vec3::new(
						x as f32,
						(DIM - 1) as f32,
						self.depths[x + (DIM - 1) * DIM]
							+ terrain.height_points()[x + (DIM - 1) * DIM],
					));
				}
				for y in (0..(DIM)).rev() {
					bord_points.push(Vec3::new(0.0, y as f32, 0.0));
					bord_points.push(Vec3::new(
						0.0,
						y as f32,
						self.depths[0 + y * DIM] + terrain.height_points()[0 + y * DIM],
					));
				}

				if let Some(ent_mesh) = store.get_mut(self.border_id) {
					if let Some(mesh) = ent_mesh.as_any().downcast_ref::<Mesh>() {
						mesh.update_vertices(|data| {
							let bord_vert = Mesh::wall_gen_vertices(&bord_points);
							for (i, val) in bord_vert.into_iter().enumerate() {
								data[i] = val;
							}
						});
					}
				}
			}
		}
	}

	fn handle_inputs(&mut self, inputs: &Inputs, store: &EntityStore) {
		if self.input_str > 0
			&& (inputs.is_just_pressed(KeyCode::Minus) || inputs.is_just_pressed(KeyCode::M))
		{
			self.input_str = self.input_str - 1;
		}
		if self.input_str < 2
			&& (inputs.is_just_pressed(KeyCode::Plus)
				|| inputs.is_just_pressed(KeyCode::Equals)
				|| inputs.is_just_pressed(KeyCode::P))
		{
			self.input_str = self.input_str + 1;
		}

		if self.avg_depth < Water::MAX_HEIGHT && inputs.is_pressed(KeyCode::W) {
			for i in 0..DIM {
				self.depths[i] += 0.5 + self.input_str as f32 * 0.5;
			}
		}

		if let Some(ent_terrain) = store.get(self.terrain_id) {
			if let Some(terrain) = ent_terrain.as_any().downcast_ref::<HeightMap>() {
				if self.avg_depth < Water::MAX_HEIGHT && inputs.is_pressed(KeyCode::T) {
					for i in 0..DIM {
						for j in 0..DIM {
							if terrain.height_points()[i + j * DIM] <= ZERO_DEPTH
								&& terrain.height_points()[i + j * DIM] >= -ZERO_DEPTH
							{
								self.depths[i + j * DIM] += 0.1 + 0.1 * self.input_str as f32;
							}
						}
					}
				}

				if inputs.is_pressed(KeyCode::D) {
					for i in 0..DIM {
						for j in 0..DIM {
							if terrain.height_points()[i + j * DIM] <= ZERO_DEPTH
								&& terrain.height_points()[i + j * DIM] >= -ZERO_DEPTH
							{
								self.depths[i + j * DIM] -= 0.1 + 0.1 * self.input_str as f32;
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
	}

	pub fn get_height(&self, store: &EntityStore, x: usize, y: usize) -> Option<f32> {
		if let Some(ent_terrain) = store.get(self.terrain_id) {
			if let Some(terrain) = ent_terrain.as_any().downcast_ref::<HeightMap>() {
				return Some(self.depths[x + y * DIM] + terrain.height_points()[x + y * DIM]);
			}
		}
		return None;
	}

	pub fn add_depth(&mut self, x: usize, y: usize, inc: f32) {
		self.depths[x + y * DIM] += inc;
	}

	pub fn get_avg_depth(&self) -> f32 {
		self.avg_depth
	}

	pub fn input_str(&self) -> u8 {
		self.input_str
	}
}

impl Entity for Water {
	fn as_any(&self) -> &dyn Any {
		self
	}

	fn as_any_mut(&mut self) -> &mut dyn Any {
		self
	}

	fn update(&mut self, delta: f32, inputs: &Inputs, store: &EntityStore) {
		self.update_pipes_flow(delta, store);
		self.limit_flows(delta);
		self.update_depths(delta);
		self.handle_inputs(inputs, store);
		self.update_mesh(store);
		self.update_border_mesh(store);
	}
}
