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
pub struct MeshPoints {
	vao: u32,
	vbo: u32,
	shader_name: String,
	count: i32,
	offset: f32,
	opaque: bool,
}

impl MeshPoints {
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
				3 * mem::size_of::<GLfloat>() as GLsizei,
				ptr::null(),
			);
			gl::EnableVertexAttribArray(0);
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

	pub fn points_vertices(points: &Vec<Vec3>) -> Vec<f32> {
		let mut vertices = Vec::new();
		for point in points {
			let vertex = [point.x, point.z, point.y];
			vertices.extend_from_slice(&vertex);
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
}

impl Renderable for MeshPoints {
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
			let uniform_loc = gl::GetUniformLocation(
				shader_program.id(),
				map_engine_error!(CString::new("viewPos"), BadCString)?
					.as_c_str()
					.as_ptr(),
			);
			gl::Uniform3f(uniform_loc, view_pos.x, view_pos.y, view_pos.z);
			gl::BindVertexArray(self.vao);
			gl::DrawArrays(gl::POINTS, 0, self.count / 3);
		}
		return Ok(());
	}
}

impl Entity for MeshPoints {
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
