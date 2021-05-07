mod camera;
mod entities;
mod error;
mod input;
mod mesh;
mod render;
mod shader_program;

pub use camera::{Camera, PolygonMode};
pub use entities::{Entity, EntityStore};
pub use error::EngineError;
pub use input::{ClickButton, Inputs};
pub use mesh::Mesh;
pub use render::{Renderable, Renderer, RendererBuilder};
pub use shader_program::ShaderProgram;

use glutin::{
	event::{Event, MouseScrollDelta, WindowEvent},
	event_loop::{ControlFlow, EventLoopProxy, EventLoopWindowTarget},
};
use std::time;

pub fn core_loop(
	mut renderer: Renderer,
	mut entities: EntityStore,
	_event_proxy: EventLoopProxy<()>,
) -> Box<dyn FnMut(Event<'_, ()>, &EventLoopWindowTarget<()>, &mut ControlFlow)> {
	let mut inputs = Inputs::new();
	let mut last_draw = time::Instant::now();
	Box::new(move |event, _target, flow: &mut ControlFlow| {
		*flow = ControlFlow::Poll;

		match event {
			Event::LoopDestroyed => return,
			Event::WindowEvent { event, .. } => match event {
				WindowEvent::CloseRequested => *flow = ControlFlow::Exit,
				WindowEvent::Resized(size) => renderer.resize(size),
				WindowEvent::KeyboardInput { input, .. } => inputs.store_key(input),
				WindowEvent::MouseInput { state, button, .. } => inputs.store_click(state, button),
				WindowEvent::CursorMoved { position, .. } => inputs.store_motion(position),
				WindowEvent::MouseWheel { delta, .. } => {
					if let MouseScrollDelta::LineDelta(x, y) = delta {
						inputs.store_mouse_wheel((x, y));
					}
				}
				_ => (),
			},
			Event::RedrawRequested(_) => {
				let delta = time::Instant::now().duration_since(last_draw).as_micros();
				entities.update((delta % 100000) as f32 / 1000000.0, &inputs);
				inputs.update();
				last_draw = time::Instant::now();
				if !entities.render(&mut renderer) {
					*flow = ControlFlow::Exit;
				}
			}
			Event::UserEvent(_) => {}
			_ => {}
		}

		if time::Instant::now().duration_since(last_draw).as_micros() >= 8333 {
			renderer.window().request_redraw();
		}
	})
}
