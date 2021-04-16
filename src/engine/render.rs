use super::ShaderProgram;
use glutin::{
	dpi::PhysicalSize,
	event_loop::EventLoop,
	window::{Window, WindowBuilder},
	ContextBuilder, ContextWrapper, PossiblyCurrent,
};
use std::collections::HashMap;

const DEFAULT_WIDTH: u32 = 800;
const DEFAULT_HEIGHT: u32 = 600;
const DEFAULT_TITLE: String = String::new();

pub trait Renderable {
	fn shader_name(&self) -> &String;
	fn draw(&self, shader_program: &ShaderProgram);
}

pub struct RendererBuilder {
	title: String,
	size: PhysicalSize<u32>,
	resizable: bool,
}

impl RendererBuilder {
	pub fn new() -> Self {
		Self {
			title: DEFAULT_TITLE,
			size: PhysicalSize {
				width: DEFAULT_WIDTH,
				height: DEFAULT_HEIGHT,
			},
			resizable: false,
		}
	}

	pub fn title(self, title: &str) -> Self {
		Self {
			title: String::from(title),
			size: self.size,
			resizable: self.resizable,
		}
	}

	pub fn size(self, size: (u32, u32)) -> Self {
		Self {
			title: self.title,
			size: PhysicalSize {
				width: size.0,
				height: size.1,
			},
			resizable: self.resizable,
		}
	}

	pub fn resizable(self, resizable: bool) -> Self {
		Self {
			title: self.title,
			size: self.size,
			resizable,
		}
	}

	pub fn build(self, event_loop: &EventLoop<()>) -> Renderer {
		let window = WindowBuilder::new()
			.with_title(self.title)
			.with_inner_size(self.size)
			.with_resizable(self.resizable);
		let gl_window = ContextBuilder::new()
			.with_vsync(true)
			.build_windowed(window, event_loop)
			.unwrap();
		let gl_window = unsafe { gl_window.make_current().unwrap() };
		gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
		unsafe {
			gl::Enable(gl::DEPTH_TEST);
		}
		Renderer {
			gl_window,
			shaders: HashMap::new(),
		}
	}
}

pub struct Renderer {
	gl_window: ContextWrapper<PossiblyCurrent, Window>,
	shaders: HashMap<String, ShaderProgram>,
}

impl Renderer {
	pub fn clear(&self) {
		unsafe {
			gl::ClearColor(0.0, 0.0, 0.0, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
		}
	}

	pub fn swap(&self) {
		self.gl_window.swap_buffers().unwrap();
	}

	pub fn draw(&self, obj: &dyn Renderable) {
		let shader_program = self.shaders.get(obj.shader_name()).unwrap();
		shader_program.use_program();
		obj.draw(shader_program);
	}

	pub fn resize(&self, size: PhysicalSize<u32>) {
		self.gl_window.resize(size);
		unsafe {
			gl::Viewport(0, 0, size.width as i32, size.height as i32);
		}
	}

	pub fn load_shader(&mut self, name: &str) {
		self.shaders
			.insert(String::from(name), ShaderProgram::new(name));
	}

	pub fn window(&self) -> &Window {
		self.gl_window.window()
	}
}
