use super::{Camera, EngineError, Entity, Renderable, Renderer};
use crate::engine_error;
use gl::types::*;
use glam::Vec3;
use std::any::Any;
use std::mem;
use std::os::raw::c_void;
use std::ptr;

#[derive(Debug)]
pub struct Mesh {
	vao: u32,
	vbo: u32,
	shader_name: String,
	count: i32,
	offset: f32,
	opaque: bool,
}

impl Mesh {
	pub fn new(
		shader_name: &str,
		vertices_flat: &Vec<f32>,
		dim: usize,
		opaque: bool,
		static_data: bool,
	) -> Self {
		let offset = (dim - 1) as f32 * 0.5;
		let count = vertices_flat.len() as i32;

		let (mut vbo, mut vao) = (0, 0);
		unsafe {
			gl::GenVertexArrays(1, &mut vao);
			gl::GenBuffers(1, &mut vbo);

			gl::BindVertexArray(vao);
			gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
			gl::BufferData(
				gl::ARRAY_BUFFER,
				(vertices_flat.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
				&vertices_flat[0] as *const f32 as *const c_void,
				if static_data {
					gl::STATIC_DRAW
				} else {
					gl::STREAM_DRAW
				},
			);
			gl::VertexAttribPointer(
				0,
				3,
				gl::FLOAT,
				gl::FALSE,
				6 * mem::size_of::<GLfloat>() as GLsizei,
				ptr::null(),
			);
			gl::EnableVertexAttribArray(0);
			gl::VertexAttribPointer(
				1,
				3,
				gl::FLOAT,
				gl::FALSE,
				6 * mem::size_of::<GLfloat>() as GLsizei,
				(3 * mem::size_of::<GLfloat>()) as *mut _,
			);
			gl::EnableVertexAttribArray(1);
			gl::BindBuffer(gl::ARRAY_BUFFER, 0);
			gl::BindVertexArray(0);
		}

		Self {
			vao,
			vbo,
			shader_name: String::from(shader_name),
			count,
			offset,
			opaque,
		}
	}

	pub fn heights_gen_vertices(dim: usize, height_pts: &Vec<f32>) -> Vec<f32> {
		let mut vertices = Vec::new();
		for i in 0..dim {
			for j in 0..dim {
				if i + 1 < dim && j + 1 < dim {
					let top_left = Mesh::compute_vertice(i, j, dim, height_pts);
					let top_right = Mesh::compute_vertice(i + 1, j, dim, height_pts);
					let bot_left = Mesh::compute_vertice(i, j + 1, dim, height_pts);
					let bot_right = Mesh::compute_vertice(i + 1, j + 1, dim, height_pts);

					// first triangle
					vertices.extend_from_slice(&top_left);
					vertices.extend_from_slice(&top_right);
					vertices.extend_from_slice(&bot_right);
					// second triangle
					vertices.extend_from_slice(&top_left);
					vertices.extend_from_slice(&bot_right);
					vertices.extend_from_slice(&bot_left);
				}
			}
		}
		return vertices;
	}

	pub fn update_vertices(&self, update: impl Fn(&mut [f32])) {
		unsafe {
			gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
			let data_ptr = gl::MapBuffer(gl::ARRAY_BUFFER, gl::WRITE_ONLY) as *mut f32;
			let data_slice = std::ptr::slice_from_raw_parts_mut(data_ptr, self.count as usize);
			let data_ref = &mut *data_slice;
			update(data_ref);
			gl::UnmapBuffer(gl::ARRAY_BUFFER);
		}
	}

	fn compute_vertice(i: usize, j: usize, dim: usize, height_pts: &Vec<f32>) -> [f32; 6] {
		let mut vertice = [0.0; 6];
		let (x, y) = (i as f32, j as f32);
		// coords
		vertice[0] = x;
		vertice[1] = height_pts[i + j * dim];
		vertice[2] = y;
		// normal
		let prev_x = if i > 0 {
			height_pts[i - 1 + j * dim]
		} else {
			height_pts[i + j * dim]
		};
		let next_x = if i < dim - 1 {
			height_pts[i + 1 + j * dim]
		} else {
			height_pts[i + j * dim]
		};
		let prev_y = if j > 0 {
			height_pts[i + (j - 1) * dim]
		} else {
			height_pts[i + j * dim]
		};
		let next_y = if j < dim - 1 {
			height_pts[i + (j + 1) * dim]
		} else {
			height_pts[i + j * dim]
		};
		let normal = Mesh::normal(prev_x, next_x, prev_y, next_y);
		vertice[3] = normal.x;
		vertice[4] = normal.z;
		vertice[5] = normal.y;
		return vertice;
	}

	pub fn wall_gen_vertices(columns: &Vec<Vec3>) -> Vec<f32> {
		let mut vertices = Vec::new();
		for i in 0..(columns.len() / 2 - 1) {
			let (bot_left, top_left, bot_right, top_right) =
				Mesh::compute_wall_vertice(i * 2, columns);

			// first triangle
			vertices.extend_from_slice(&top_left);
			vertices.extend_from_slice(&bot_right);
			vertices.extend_from_slice(&top_right);
			// second triangle
			vertices.extend_from_slice(&top_left);
			vertices.extend_from_slice(&bot_left);
			vertices.extend_from_slice(&bot_right);
		}
		return vertices;
	}

	fn compute_wall_vertice(
		index: usize,
		columns: &Vec<Vec3>,
	) -> ([f32; 6], [f32; 6], [f32; 6], [f32; 6]) {
		let mut bot_left = [0.0; 6];
		let mut top_left = [0.0; 6];
		let mut bot_right = [0.0; 6];
		let mut top_right = [0.0; 6];

		let normal =
			(columns[index + 1] - columns[index]).cross(columns[index + 2] - columns[index]);

		bot_left[0] = columns[index].x;
		bot_left[1] = columns[index].z;
		bot_left[2] = columns[index].y;
		bot_left[3] = normal.x;
		bot_left[4] = normal.z;
		bot_left[5] = normal.y;

		top_left[0] = columns[index + 1].x;
		top_left[1] = columns[index + 1].z;
		top_left[2] = columns[index + 1].y;
		top_left[3] = normal.x;
		top_left[4] = normal.z;
		top_left[5] = normal.y;

		bot_right[0] = columns[index + 2].x;
		bot_right[1] = columns[index + 2].z;
		bot_right[2] = columns[index + 2].y;
		bot_right[3] = normal.x;
		bot_right[4] = normal.z;
		bot_right[5] = normal.y;

		top_right[0] = columns[index + 3].x;
		top_right[1] = columns[index + 3].z;
		top_right[2] = columns[index + 3].y;
		top_right[3] = normal.x;
		top_right[4] = normal.z;
		top_right[5] = normal.y;

		return (bot_left, top_left, bot_right, top_right);
	}

	pub fn normal(left: f32, right: f32, top: f32, bottom: f32) -> Vec3 {
		Vec3::new(2.0 * (right - left), 2.0 * (bottom - top), -4.0).normalize()
	}
}

impl Renderable for Mesh {
	fn is_opaque(&self) -> bool {
		self.opaque
	}

	fn draw(&self, renderer: &Renderer, camera: &Camera) -> Result<(), EngineError> {
		let shader_program = renderer.get_shader(&self.shader_name).ok_or(engine_error!(
			ShaderFail,
			format!("Shader '{}' is missing", self.shader_name)
		))?;
		let pos = glam::Vec3::new(-self.offset, 0.0, -self.offset);
		let model = glam::Mat4::from_scale_rotation_translation(
			glam::Vec3::new(1.0, 1.0, 1.0),
			glam::Quat::from_axis_angle(glam::Vec3::Y, 0.0),
			pos,
		);
		let view = camera.view();
		let projection = camera.perspective();
		let view_pos = camera.pos();
		shader_program.use_program();
		shader_program.load_uniform_matrix_4fv("model", model)?;
		shader_program.load_uniform_matrix_4fv("view", view)?;
		shader_program.load_uniform_matrix_4fv("projection", projection)?;
		shader_program.load_uniform_3fv("viewPos", view_pos)?;
		unsafe {
			gl::BindVertexArray(self.vao);
			gl::DrawArrays(gl::TRIANGLES, 0, self.count);
		}
		return Ok(());
	}
}

impl Entity for Mesh {
	fn as_renderable(&self) -> Option<&dyn Renderable> {
		return Some(self);
	}

	fn as_any(&self) -> &dyn Any {
		self
	}

	fn as_any_mut(&mut self) -> &mut dyn Any {
		self
	}
}
