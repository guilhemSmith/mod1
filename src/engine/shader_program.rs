use gl::types::*;
use glam::{Mat4, Vec3};

use super::EngineError;
use crate::{engine_error, map_engine_error};
use std::ffi::CString;
use std::fs::read;
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
	pub fn new(name: &str, light: bool) -> Result<Self, EngineError> {
		let vertex_shader = compile_shader(name, ShaderType::Vertex)?;
		let fragment_shader = compile_shader(name, ShaderType::Fragment)?;
		let light_shader = if light {
			Some(compile_shader("light", ShaderType::Fragment)?)
		} else {
			None
		};
		let shader_program = link_shaders(vertex_shader, light_shader, fragment_shader)?;
		Ok(ShaderProgram { id: shader_program })
	}
	pub fn use_program(&self) {
		unsafe {
			gl::UseProgram(self.id);
		}
	}

	pub fn id(&self) -> u32 {
		self.id
	}

	pub fn load_uniform_matrix_4fv(&self, name: &str, matrix: Mat4) -> Result<(), EngineError> {
		unsafe {
			let uniform_loc = gl::GetUniformLocation(
				self.id,
				map_engine_error!(CString::new(name), BadCString)?
					.as_c_str()
					.as_ptr(),
			);
			gl::UniformMatrix4fv(uniform_loc, 1, gl::FALSE, matrix.as_ref().as_ptr());
		}
		return Ok(());
	}

	pub fn load_uniform_3fv(&self, name: &str, vec: Vec3) -> Result<(), EngineError> {
		unsafe {
			let uniform_loc = gl::GetUniformLocation(
				self.id,
				map_engine_error!(CString::new(name), BadCString)?
					.as_c_str()
					.as_ptr(),
			);
			gl::Uniform3f(uniform_loc, vec.x, vec.y, vec.z);
		}
		return Ok(());
	}
}

fn compile_shader(name: &str, shader_type: ShaderType) -> Result<u32, EngineError> {
	let mut path = std::path::PathBuf::from(env!("OUT_DIR"));
	path.push("shaders");
	path.push(name);
	path.push(shader_type.str());
	path.set_extension("glsl");
	let err_msg = format!("Failed to read file '{}'", path.to_string_lossy());
	let source = map_engine_error!(read(&path), FileError, err_msg)?;
	let c_str = map_engine_error!(CString::new(source), BadCString)?;
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
			let err_msg = format!(
				"ERROR::SHADER::{}::COMPILATION_FAILED\n{}",
				shader_type.str().to_uppercase(),
				String::from_utf8_lossy(&info_log)
			);
			Err(engine_error!(ShaderFail, err_msg))
		}
	}
}

fn link_shaders(
	vertex_shader: u32,
	light_shader: Option<u32>,
	fragment_shader: u32,
) -> Result<u32, EngineError> {
	unsafe {
		let shader_program = gl::CreateProgram();
		gl::AttachShader(shader_program, vertex_shader);
		if let Some(light) = light_shader {
			gl::AttachShader(shader_program, light);
		}
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
			let err_msg = format!(
				"ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}",
				String::from_utf8_lossy(&info_log)
			);
			Err(engine_error!(ShaderFail, err_msg))
		}
	}
}
