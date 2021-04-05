use gl::types::*;

use std::ffi::CString;
use std::ptr;
use std::str;

enum ShaderType {
	Vertex,
	Fragment,
}

impl ShaderType {
	pub fn str(&self) -> &str {
		match self {
			ShaderType::Vertex => "vertex",
			ShaderType::Fragment => "fragment",
		}
	}

	pub fn gl(&self) -> u32 {
		match self {
			ShaderType::Vertex => gl::VERTEX_SHADER,
			ShaderType::Fragment => gl::FRAGMENT_SHADER,
		}
	}
}

pub struct ShaderProgram {
	id: u32,
}

impl ShaderProgram {
	pub fn new(name: &str) -> Self {
		let vertex_shader = compile_shader(name, ShaderType::Vertex).unwrap();
		let fragment_shader = compile_shader(name, ShaderType::Fragment).unwrap();
		let shader_program = link_shaders(vertex_shader, fragment_shader).unwrap();
		ShaderProgram { id: shader_program }
	}
	pub fn use_program(&self) {
		unsafe {
			gl::UseProgram(self.id);
		}
	}
}

fn compile_shader(name: &str, shader_type: ShaderType) -> Result<u32, String> {
	let source = std::fs::read(&format!("shaders/{}/{}.glsl", name, shader_type.str())).unwrap();
	let c_str = CString::new(source).unwrap();
	unsafe {
		let mut success = i32::from(gl::FALSE);
		let mut info_log = Vec::with_capacity(512);
		info_log.set_len(512 - 1);
		let shader = gl::CreateShader(shader_type.gl());
		gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
		gl::CompileShader(shader);
		gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
		if success == i32::from(gl::TRUE) {
			Ok(shader)
		} else {
			gl::GetShaderInfoLog(
				shader,
				512,
				ptr::null_mut(),
				info_log.as_mut_ptr() as *mut GLchar,
			);
			Err(format!(
				"ERROR::SHADER::{}::COMPILATION_FAILED\n{}",
				shader_type.str().to_uppercase(),
				str::from_utf8(&info_log).unwrap()
			))
		}
	}
}

fn link_shaders(vertex_shader: u32, fragment_shader: u32) -> Result<u32, String> {
	unsafe {
		let shader_program = gl::CreateProgram();
		gl::AttachShader(shader_program, vertex_shader);
		gl::AttachShader(shader_program, fragment_shader);
		gl::LinkProgram(shader_program);
		gl::DeleteShader(vertex_shader);
		gl::DeleteShader(fragment_shader);
		let mut success = i32::from(gl::FALSE);
		let mut info_log = Vec::with_capacity(512);
		info_log.set_len(512 - 1);
		gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
		if success == i32::from(gl::TRUE) {
			Ok(shader_program)
		} else {
			gl::GetProgramInfoLog(
				shader_program,
				512,
				ptr::null_mut(),
				info_log.as_mut_ptr() as *mut GLchar,
			);
			Err(format!(
				"ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}",
				str::from_utf8(&info_log).unwrap()
			))
		}
	}
}
