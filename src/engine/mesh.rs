use super::Renderable;
use super::ShaderProgram;
use gl::types::*;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::time::SystemTime;

pub struct Mesh {
	vao: u32,
	shader_name: String,
	start: SystemTime,
}

impl Mesh {
	pub fn new(shader_name: &str) -> Self {
		let vertices: [f32; 18] = [
			-0.5, -0.5, 0.0, 1.0, 0.0, 0.0, // right
			0.5, -0.5, 0.0, 0.0, 1.0, 0.0, // top
			0.0, 0.5, 0.0, 0.0, 0.0, 1.0,
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
			start: SystemTime::now(),
		}
	}
}

impl Renderable for Mesh {
	fn shader_name(&self) -> &String {
		&self.shader_name
	}

	fn draw(&self, shader_program: &ShaderProgram) {
		let trans = glam::Mat4::from_scale_rotation_translation(
			glam::Vec3::new(0.5, 0.5, 0.5),
			glam::Quat::from_axis_angle(
				glam::Vec3::Z,
				f32::to_radians(
					SystemTime::now()
						.duration_since(self.start)
						.unwrap()
						.as_millis() as f32 * 0.05,
				),
			),
			glam::Vec3::ZERO,
		);
		unsafe {
			let transform_loc = gl::GetUniformLocation(
				shader_program.id(),
				std::ffi::CString::new("transform")
					.unwrap()
					.as_c_str()
					.as_ptr(),
			);
			gl::UniformMatrix4fv(transform_loc, 1, gl::FALSE, trans.as_ref().as_ptr());
			gl::BindVertexArray(self.vao);
			gl::DrawArrays(gl::TRIANGLES, 0, 3);
		}
	}
}
