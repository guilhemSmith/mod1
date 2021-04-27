use super::{Camera, EngineError, Entity, Renderable, Renderer};
use crate::{engine_error, map_engine_error};
use gl::types::*;
use glam::Vec3;
use std::any::Any;
use std::ffi::CString;
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
}

impl Mesh {
	pub fn new(shader_name: &str, points: &Vec<f32>, dim: usize) -> Self {
		let offset = (dim - 1) as f32 * 0.5;
		let vertices_flat: Vec<f32> = Mesh::gen_vertices(100, points);
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
				gl::STATIC_DRAW,
			);
			gl::EnableVertexAttribArray(0);
			gl::VertexAttribPointer(
				0,
				3,
				gl::FLOAT,
				gl::FALSE,
				3 * mem::size_of::<GLfloat>() as GLsizei,
				ptr::null(),
			);
			gl::BindBuffer(gl::ARRAY_BUFFER, 0);
			gl::BindVertexArray(0);
		}

		Self {
			vao,
			vbo,
			shader_name: String::from(shader_name),
			count,
			offset,
		}
	}

	pub fn gen_vertices(dim: usize, height_pts: &Vec<f32>) -> Vec<f32> {
		let mut vertices = Vec::new();
		let one = 1.0;
		for i in 0..dim {
			let x = i as f32;
			for j in 0..dim {
				let y = j as f32;
				if i + 1 < dim && j + 1 < dim {
					let top_left = vec![x, height_pts[i + j * dim], y];
					let top_right = vec![x + one, height_pts[i + 1 + j * dim], y];
					let bot_left = vec![x, height_pts[i + (j + 1) * dim], y + one];
					let bot_right = vec![x + one, height_pts[i + 1 + (j + 1) * dim], y + one];

					if (top_left[1] - bot_right[1]).abs() > (top_right[1] - bot_left[1]).abs() {
						// first triangle
						vertices.extend_from_slice(&top_left);
						vertices.extend_from_slice(&top_right);
						vertices.extend_from_slice(&bot_right);
						// second triangle
						vertices.extend_from_slice(&top_left);
						vertices.extend_from_slice(&bot_left);
						vertices.extend_from_slice(&bot_right);
					} else {
						// first triangle
						vertices.extend_from_slice(&top_right);
						vertices.extend_from_slice(&top_left);
						vertices.extend_from_slice(&bot_left);
						// second triangle
						vertices.extend_from_slice(&top_right);
						vertices.extend_from_slice(&bot_right);
						vertices.extend_from_slice(&bot_left);
					}
				}
			}
		}
		return vertices;
	}
}

impl Renderable for Mesh {
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
		shader_program.use_program();
		unsafe {
			let uniform_loc = gl::GetUniformLocation(
				shader_program.id(),
				map_engine_error!(CString::new("model"), BadCString)?
					.as_c_str()
					.as_ptr(),
			);
			gl::UniformMatrix4fv(uniform_loc, 1, gl::FALSE, model.as_ref().as_ptr());
			let uniform_loc = gl::GetUniformLocation(
				shader_program.id(),
				map_engine_error!(CString::new("view"), BadCString)?
					.as_c_str()
					.as_ptr(),
			);
			gl::UniformMatrix4fv(uniform_loc, 1, gl::FALSE, view.as_ref().as_ptr());
			let uniform_loc = gl::GetUniformLocation(
				shader_program.id(),
				map_engine_error!(CString::new("projection"), BadCString)?
					.as_c_str()
					.as_ptr(),
			);
			gl::UniformMatrix4fv(uniform_loc, 1, gl::FALSE, projection.as_ref().as_ptr());
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
}
