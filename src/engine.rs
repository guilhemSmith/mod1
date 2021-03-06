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
	mut renderer: Renderer,
	mut entities: EntityStore,
	_event_proxy: EventLoopProxy<()>,
) -> Box<dyn FnMut(Event<'_, ()>, &EventLoopWindowTarget<()>, &mut ControlFlow)> {
	let mut inputs = Inputs::new();
	let mut last_draw = time::Instant::now();
	let mut last_update = time::Instant::now();
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
				let delta = f32::min(
					8333.0,
					time::Instant::now().duration_since(last_draw).as_micros() as f32,
				) / 100000.0;
				last_draw = time::Instant::now();
				if !renderer.run(&mut entities, delta) {
					*flow = ControlFlow::Exit;
				}
			}
			Event::UserEvent(_) => {}
			Event::MainEventsCleared => {
				let delta = f32::min(
					8333.0,
					time::Instant::now().duration_since(last_update).as_micros() as f32,
				) / 100000.0;
				if delta >= 0.08333 {
					entities.update(delta, &inputs);
					inputs.update();
					last_update = time::Instant::now();
				}
				if time::Instant::now().duration_since(last_draw).as_micros() >= 8333 {
					renderer.window().request_redraw();
				}
			}
			_ => {}
		}
	})
}
