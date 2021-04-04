pub mod shader_program;

use glutin::{dpi::PhysicalSize, window::Window, ContextWrapper, PossiblyCurrent};

pub fn clear(gl_window: &ContextWrapper<PossiblyCurrent, Window>, shader_program: u32, vao: u32) {
	unsafe {
		gl::ClearColor(0.0, 0.0, 0.0, 1.0);
		gl::Clear(gl::COLOR_BUFFER_BIT);
		gl::UseProgram(shader_program);
		gl::BindVertexArray(vao);
		gl::DrawArrays(gl::TRIANGLES, 0, 3);
	}
	gl_window.swap_buffers().unwrap();
}

pub fn resize(gl_window: &ContextWrapper<PossiblyCurrent, Window>, size: PhysicalSize<u32>) {
	gl_window.resize(size)
}
