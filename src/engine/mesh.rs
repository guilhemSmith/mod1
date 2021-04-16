use super::{Camera, Renderable, ShaderProgram};
use gl::types::*;
use std::mem;
use std::os::raw::c_void;
use std::ptr;

pub struct Mesh {
	vao: u32,
	shader_name: String,
}

impl Mesh {
	pub fn new(shader_name: &str) -> Self {
		let vertices: [f32; 36] = [
			-1.0, 0.0, -1.0, 0.0, 0.8, 0.0, // bot left
			1.0, 0.0, -1.0, 0.0, 0.8, 0.0, // bot right
			1.0, 0.0, 1.0, 0.0, 0.8, 0.0, // top right
			1.0, 0.0, 1.0, 0.0, 0.8, 0.0, // top right
			-1.0, 0.0, 1.0, 0.0, 0.8, 0.0, // top left
			-1.0, 0.0, -1.0, 0.0, 0.8, 0.0, // bot left
		];

		let (mut vbo, mut vao) = (0, 0);
		unsafe {
			gl::GenVertexArrays(1, &mut vao);
			gl::GenBuffers(1, &mut vbo);
			gl::BindVertexArray(vao);
			gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
			gl::BufferData(
				gl::ARRAY_BUFFER,
				(vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
				&vertices[0] as *const f32 as *const c_void,
				gl::STATIC_DRAW,
			);
			gl::EnableVertexAttribArray(0);
			gl::VertexAttribPointer(
				0,
				3,
				gl::FLOAT,
				gl::FALSE,
				6 * mem::size_of::<GLfloat>() as GLsizei,
				ptr::null(),
			);
			gl::EnableVertexAttribArray(1);
			gl::VertexAttribPointer(
				1,
				3,
				gl::FLOAT,
				gl::FALSE,
				6 * mem::size_of::<GLfloat>() as GLsizei,
				(3 * mem::size_of::<GLfloat>()) as *const c_void,
			);
			gl::BindBuffer(gl::ARRAY_BUFFER, 0);
			gl::BindVertexArray(0);
		}

		Self {
			vao,
			shader_name: String::from(shader_name),
		}
	}
}

impl Renderable for Mesh {
	fn shader_name(&self) -> &String {
		&self.shader_name
	}

	fn draw(&self, shader_program: &ShaderProgram, camera: &Camera) {
		let pos = glam::Vec3::ZERO;
		let model = glam::Mat4::from_scale_rotation_translation(
			glam::Vec3::new(0.5, 0.5, 0.5),
			glam::Quat::from_axis_angle(glam::Vec3::Y, 0.0),
			pos,
		);
		let view = camera.view();
		let projection = camera.perspective();
		unsafe {
			let uniform_loc = gl::GetUniformLocation(
				shader_program.id(),
				std::ffi::CString::new("model").unwrap().as_c_str().as_ptr(),
			);
			gl::UniformMatrix4fv(uniform_loc, 1, gl::FALSE, model.as_ref().as_ptr());
			let uniform_loc = gl::GetUniformLocation(
				shader_program.id(),
				std::ffi::CString::new("view").unwrap().as_c_str().as_ptr(),
			);
			gl::UniformMatrix4fv(uniform_loc, 1, gl::FALSE, view.as_ref().as_ptr());
			let uniform_loc = gl::GetUniformLocation(
				shader_program.id(),
				std::ffi::CString::new("projection")
					.unwrap()
					.as_c_str()
					.as_ptr(),
			);
			gl::UniformMatrix4fv(uniform_loc, 1, gl::FALSE, projection.as_ref().as_ptr());
			gl::BindVertexArray(self.vao);
			gl::DrawArrays(gl::TRIANGLES, 0, 6);
		}
	}
}
