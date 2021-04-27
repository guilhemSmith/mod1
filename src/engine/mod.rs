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
	event_loop::{ControlFlow, EventLoopWindowTarget},
};
use std::time;

pub fn core_loop(
	mut renderer: Renderer,
	mut entities: EntityStore,
) -> Box<dyn FnMut(Event<'_, ()>, &EventLoopWindowTarget<()>, &mut ControlFlow)> {
	let mut inputs = Inputs::new();
	let mut last_draw = time::Instant::now();
	Box::new(move |event, _target, flow: &mut ControlFlow| {
		*flow = ControlFlow::Poll;

		let delta = time::Instant::now().duration_since(last_draw).as_micros();
		let mut logic_frame = if delta >= 8333 {
			renderer.window().request_redraw();
			let _fps = 1000000 / delta;
			// println!("{}", _fps);
			true
		} else {
			false
		};

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
				last_draw = time::Instant::now();
				if !entities.render(&mut renderer) {
					logic_frame = false;
					*flow = ControlFlow::Exit;
				}
			}
			_ => {}
		}
		if logic_frame {
			entities.update(delta as f32 / 1000000.0, &inputs);
			inputs.update();
		}
	})
}
