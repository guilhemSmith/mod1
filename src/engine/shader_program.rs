use gl::types::*;

use std::ffi::CString;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::str;

pub fn compile(name: &str) -> (u32, u32) {
	return unsafe {
		// Setup shader compilation checks
		let mut success = i32::from(gl::FALSE);
		let mut info_log = Vec::with_capacity(512);
		info_log.set_len(512 - 1); // -1 to skip trialing null character

		// Vertex shader
		let vertex_source = std::fs::read(&format!("shaders/{}/vertex.glsl", name)).unwrap();
		let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
		let c_str_vert = CString::new(vertex_source).unwrap();
		gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
		gl::CompileShader(vertex_shader);

		// Check for shader compilation errors
		gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
		if success != i32::from(gl::TRUE) {
			gl::GetShaderInfoLog(
				vertex_shader,
				512,
				ptr::null_mut(),
				info_log.as_mut_ptr() as *mut GLchar,
			);
			println!(
				"ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}",
				str::from_utf8(&info_log).unwrap()
			);
		}

		// Fragment shader
		let fragment_source = std::fs::read(&format!("shaders/{}/fragment.glsl", name)).unwrap();
		let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
		let c_str_frag = CString::new(fragment_source).unwrap();
		gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
		gl::CompileShader(fragment_shader);

		// Check for shader compilation errors
		gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
		if success != i32::from(gl::TRUE) {
			gl::GetShaderInfoLog(
				fragment_shader,
				512,
				ptr::null_mut(),
				info_log.as_mut_ptr() as *mut GLchar,
			);
			println!(
				"ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}",
				str::from_utf8(&info_log).unwrap()
			);
		}

		// Link Shaders
		let shader_program = gl::CreateProgram();
		gl::AttachShader(shader_program, vertex_shader);
		gl::AttachShader(shader_program, fragment_shader);
		gl::LinkProgram(shader_program);

		// Check for linking errors
		gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
		if success != i32::from(gl::TRUE) {
			gl::GetProgramInfoLog(
				shader_program,
				512,
				ptr::null_mut(),
				info_log.as_mut_ptr() as *mut GLchar,
			);
			println!(
				"ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}",
				str::from_utf8(&info_log).unwrap()
			);
		}
		gl::DeleteShader(vertex_shader);
		gl::DeleteShader(fragment_shader);

		let vertices: [f32; 18] = [
			-0.5, -0.5, 0.0, 1.0, 0.0, 0.0, // right
			0.5, -0.5, 0.0, 0.0, 1.0, 0.0, // top
			0.0, 0.5, 0.0, 0.0, 0.0, 1.0,
		];

		let (mut vbo, mut vao) = (0, 0);
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

		(shader_program, vao)
	};
}
