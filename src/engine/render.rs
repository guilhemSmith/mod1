use super::{Camera, EngineError, EntityStore, ShaderProgram};
use crate::map_engine_error;
use glutin::{
	dpi::PhysicalSize,
	event_loop::EventLoop,
	window::{Window, WindowBuilder},
	ContextBuilder, ContextWrapper, PossiblyCurrent,
};
use std::collections::HashMap;
use std::fmt::Debug;

const DEFAULT_WIDTH: u32 = 800;
const DEFAULT_HEIGHT: u32 = 600;
const DEFAULT_TITLE: String = String::new();

pub trait Renderable: Debug {
	fn draw(&self, renderer: &Renderer, camera: &Camera) -> Result<(), EngineError>;
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

	pub fn build(self, event_loop: &EventLoop<()>) -> Result<Renderer, EngineError> {
		let window = WindowBuilder::new()
			.with_title(self.title)
			.with_inner_size(self.size)
			.with_resizable(self.resizable);
		let err_msg = String::from("Failed to build gl window wrapper");
		let res = ContextBuilder::new()
			.with_vsync(true)
			.build_windowed(window, event_loop);
		let gl_window = map_engine_error!(res, GLError, err_msg)?;
		let err_msg = String::from("Failed to make window current");
		let gl_window = unsafe { map_engine_error!(gl_window.make_current(), GLError, err_msg)? };
		gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
		unsafe {
			gl::Enable(gl::DEPTH_TEST);
			gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
		}
		Ok(Renderer {
			gl_window,
			shaders: HashMap::new(),
			cam_key: None,
		})
	}
}

pub struct Renderer {
	gl_window: ContextWrapper<PossiblyCurrent, Window>,
	shaders: HashMap<String, ShaderProgram>,
	cam_key: Option<u128>,
}

impl Renderer {
	pub fn set_cam(&mut self, key: Option<u128>) {
		self.cam_key = key;
	}

	pub fn clear(&self) {
		unsafe {
			gl::ClearColor(0.0, 0.0, 0.0, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
		}
	}

	pub fn swap(&self) -> Result<(), EngineError> {
		let err_msg = String::from("Failed to swap GL buffer");
		map_engine_error!(self.gl_window.swap_buffers(), GLError, err_msg)
	}

	pub fn render(&self, obj: &dyn Renderable, entities: &EntityStore) {
		if let Some(key) = self.cam_key {
			if let Some(camera_entity) = entities.get(key) {
				if let Some(camera) = camera_entity.as_any().downcast_ref::<Camera>() {
					if let Err(err) = obj.draw(self, camera) {
						println!("{}", err);
					};
				}
			}
		}
	}

	pub fn resize(&self, size: PhysicalSize<u32>) {
		self.gl_window.resize(size);
		unsafe {
			gl::Viewport(0, 0, size.width as i32, size.height as i32);
		}
	}

	pub fn load_shader(&mut self, name: &str) {
		match ShaderProgram::new(name) {
			Ok(shader) => {
				self.shaders.insert(String::from(name), shader);
			}
			Err(err) => {
				println!("{}", err);
			}
		}
	}

	pub fn get_shader(&self, name: &str) -> Option<&ShaderProgram> {
		self.shaders.get(name)
	}

	pub fn window(&self) -> &Window {
		self.gl_window.window()
	}
}
