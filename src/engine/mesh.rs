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
	shader_name: String,
	count: i32,
	offset: f32,
}

impl Mesh {
	pub fn new(shader_name: &str, vertices: &Vec<Vec3>, offset: f32) -> Self {
		let count = vertices.len() as i32 * 3;
		let vertices_flat: Vec<f32> = vertices
			.iter()
			.flat_map(|v3| vec![v3.x, v3.y, v3.z])
			.collect();

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
			shader_name: String::from(shader_name),
			count,
			offset,
		}
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
