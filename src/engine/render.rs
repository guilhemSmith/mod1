use super::{Camera, EngineError, EntityStore, MeshPoints, ShaderProgram};
use crate::{engine_error, map_engine_error};
use gl::types::*;
use glam::Vec3;
use glutin::{
	dpi::PhysicalSize,
	event_loop::EventLoop,
	window::{Window, WindowBuilder},
	ContextBuilder, ContextWrapper, PossiblyCurrent,
};
use std::collections::hash_set;
use std::collections::HashMap;
use std::fmt::Debug;
use std::ptr;

const DEFAULT_WIDTH: u32 = 800;
const DEFAULT_HEIGHT: u32 = 600;
const DEFAULT_TITLE: String = String::new();

pub trait RenderableStore {
	fn opaques_renderables_keys(&self) -> hash_set::Iter<u128>;
	fn transparent_renderables_keys(&self) -> hash_set::Iter<u128>;
}

pub trait Renderable: Debug {
	fn is_opaque(&self) -> bool {
		true
	}
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
		let (mut fbo, mut tcb, mut rbo) = (0, 0, 0);
		unsafe {
			gl::Enable(gl::DEPTH_TEST);
			gl::Enable(gl::BLEND);
			gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
			gl::Enable(gl::CULL_FACE);
			gl::CullFace(gl::FRONT);
			gl::FrontFace(gl::CCW);

			gl::PointSize(2.0);

			gl::GenFramebuffers(1, &mut fbo);
			gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

			gl::GenTextures(1, &mut tcb);
			gl::BindTexture(gl::TEXTURE_2D, tcb);
			gl::TexImage2D(
				gl::TEXTURE_2D,
				0,
				gl::DEPTH_COMPONENT24 as GLint,
				self.size.width as GLint,
				self.size.height as GLint,
				0,
				gl::DEPTH_COMPONENT,
				gl::FLOAT,
				ptr::null(),
			);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);

			gl::GenRenderbuffers(1, &mut rbo);
			gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
			gl::RenderbufferStorage(
				gl::RENDERBUFFER,
				gl::DEPTH_COMPONENT,
				self.size.width as GLint,
				self.size.height as GLint,
			);
			gl::FramebufferRenderbuffer(
				gl::FRAMEBUFFER,
				gl::DEPTH_ATTACHMENT,
				gl::RENDERBUFFER,
				rbo,
			);

			gl::FramebufferTexture(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, tcb, 0);
			let db = gl::DEPTH_ATTACHMENT;
			gl::DrawBuffers(1, &db);

			if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
				eprintln!("ERROR::FRAMEBUFFER:: Framebuffer is not complete!");
			}
			gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
		}
		Ok(Renderer {
			gl_window,
			shaders: HashMap::new(),
			cam_key: None,
			opaque_frame_buffer: fbo,
			opaque_screen_text: tcb,
		})
	}
}

pub struct Renderer {
	gl_window: ContextWrapper<PossiblyCurrent, Window>,
	shaders: HashMap<String, ShaderProgram>,
	cam_key: Option<u128>,
	opaque_frame_buffer: u32,
	opaque_screen_text: u32,
}

impl Renderer {
	pub fn run(&self, store: &EntityStore) -> bool {
		unsafe {
			gl::BindFramebuffer(gl::FRAMEBUFFER, self.opaque_frame_buffer);
		}
		self.clear();
		for key in store.opaques_renderables_keys() {
			if let Some(entity) = store.get(*key) {
				if let Some(renderable) = entity.as_renderable() {
					if let Err(err) = self.render(renderable, store) {
						eprintln!("{}", err);
						return false;
					}
				}
			}
		}
		unsafe {
			gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
		}
		self.clear();
		for key in store.opaques_renderables_keys() {
			if let Some(entity) = store.get(*key) {
				if let Some(renderable) = entity.as_renderable() {
					if let Err(err) = self.render(renderable, store) {
						eprintln!("{}", err);
						return false;
					}
				}
			}
		}
		unsafe {
			gl::BindTexture(gl::TEXTURE_2D, self.opaque_screen_text);
		}
		for key in store.transparent_renderables_keys() {
			if let Some(entity) = store.get(*key) {
				if let Some(renderable) = entity.as_renderable() {
					if let Err(err) = self.render(renderable, store) {
						eprintln!("{}", err);
						return false;
					}
				}
			}
		}
		return if let Err(err) = self.swap() {
			eprintln!("{}", err);
			false
		} else {
			true
		};
	}

	pub fn set_cam(&mut self, key: Option<u128>) {
		self.cam_key = key;
	}

	fn clear(&self) {
		unsafe {
			gl::ClearColor(0.0, 0.0, 0.0, 1.0);
			gl::ClearDepth(1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
		}
	}

	fn swap(&self) -> Result<(), EngineError> {
		let err_msg = String::from("Failed to swap GL buffer");
		map_engine_error!(self.gl_window.swap_buffers(), GLError, err_msg)
	}

	fn render(&self, obj: &dyn Renderable, entities: &EntityStore) -> Result<(), EngineError> {
		if let Some(key) = self.cam_key {
			if let Some(camera_entity) = entities.get(key) {
				if let Some(camera) = camera_entity.as_any().downcast_ref::<Camera>() {
					obj.draw(self, camera)
				} else {
					Err(engine_error!(MissingEntity, key.to_string()))
				}
			} else {
				Err(engine_error!(MissingEntity, key.to_string()))
			}
		} else {
			Ok(())
		}
	}

	pub fn resize(&self, size: PhysicalSize<u32>) {
		self.gl_window.resize(size);
		unsafe {
			gl::Viewport(0, 0, size.width as i32, size.height as i32);
		}
	}

	pub fn load_shader(&mut self, name: &str, light: bool) {
		match ShaderProgram::new(name, light) {
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

	pub fn light_pos(&self) -> Vec3 {
		Vec3::new(75.0, 60.0, 0.0)
	}

	pub fn light_source(&self, dim: usize) -> MeshPoints {
		MeshPoints::new(
			"sun",
			&MeshPoints::points_vertices(&vec![glam::Vec3::new(125.0, 60.0, 50.0)]),
			dim,
			true,
			true,
		)
	}
}
