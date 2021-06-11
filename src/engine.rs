mod camera;
mod entities;
mod error;
mod input;
mod mesh;
mod mesh_points;
mod render;
mod shader_program;

pub use camera::{Camera, PolygonMode};
pub use entities::{Entity, EntityStore};
pub use error::EngineError;
pub use input::{ClickButton, Inputs, KeyCode};
pub use mesh::Mesh;
pub use mesh_points::MeshPoints;
pub use render::{Renderable, Renderer, RendererBuilder};
pub use shader_program::ShaderProgram;

use glutin::{
	event::{Event, MouseScrollDelta, VirtualKeyCode, WindowEvent},
	event_loop::{ControlFlow, EventLoopProxy, EventLoopWindowTarget},
};
use std::time;

pub fn core_loop(
	renderer: Renderer,
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
				WindowEvent::KeyboardInput { input, .. } => {
					if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
						*flow = ControlFlow::Exit;
					}
					inputs.store_key(input)
				}
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
				entities.update(f32::min(10000.0, delta as f32) / 1000000.0, &inputs);
				inputs.update();
				last_draw = time::Instant::now();
				if !renderer.run(&mut entities) {
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