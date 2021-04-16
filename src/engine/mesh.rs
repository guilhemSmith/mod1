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
			start: SystemTime::now(),
		}
	}
}

impl Renderable for Mesh {
	fn shader_name(&self) -> &String {
		&self.shader_name
	}

	fn draw(&self, shader_program: &ShaderProgram) {
		let pos = glam::Vec3::ZERO;
		let model = glam::Mat4::from_scale_rotation_translation(
			glam::Vec3::new(0.5, 0.5, 0.5),
			glam::Quat::from_axis_angle(glam::Vec3::Y, 0.0),
			pos,
		);
		let cam_speed = 3.0;
		let pitch: f32 = 45.0;
		let yaw: f32 = (SystemTime::now()
			.duration_since(self.start)
			.unwrap()
			.as_millis() as f32
			* cam_speed)
			.to_radians();
		let dist = 2.0;
		let cam_pos = glam::Vec3::new(
			yaw.to_radians().cos() * pitch.to_radians().cos(),
			pitch.to_radians().sin(),
			yaw.to_radians().sin() * pitch.to_radians().cos(),
		)
		.normalize() * dist;
		let view = glam::Mat4::look_at_rh(cam_pos, pos, glam::Vec3::Y);
		let projection =
			glam::Mat4::perspective_rh_gl(f32::to_radians(90.0), 16.0 / 9.0, 0.1, 100.0);
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
