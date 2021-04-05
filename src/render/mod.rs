pub mod shader_program;

use glutin::{
	dpi::PhysicalSize,
	event_loop::EventLoop,
	window::{Window, WindowBuilder},
	ContextBuilder, ContextWrapper, PossiblyCurrent,
};

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

pub struct Renderer {
	gl_window: ContextWrapper<PossiblyCurrent, Window>,
}

impl Renderer {
	pub fn new(event_loop: &EventLoop<()>) -> Self {
		let window = WindowBuilder::new()
			.with_title("Glutin Triangle")
			.with_inner_size(PhysicalSize {
				width: SCREEN_WIDTH,
				height: SCREEN_HEIGHT,
			});
		let gl_window = ContextBuilder::new()
			.with_vsync(true)
			.build_windowed(window, &event_loop)
			.unwrap();
		let gl_window = unsafe { gl_window.make_current().unwrap() };
		gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

		Self { gl_window }
	}

	pub fn clear(&self) {
		unsafe {
			gl::ClearColor(0.0, 0.0, 0.0, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT);
		}
	}

	pub fn swap(&self) {
		self.gl_window.swap_buffers().unwrap();
	}
	pub fn draw(&self, shader_program: u32, vao: u32) {
		unsafe {
			gl::UseProgram(shader_program);
			gl::BindVertexArray(vao);
			gl::DrawArrays(gl::TRIANGLES, 0, 3);
		}
	}

	pub fn resize(&self, size: PhysicalSize<u32>) {
		self.gl_window.resize(size)
	}
}
