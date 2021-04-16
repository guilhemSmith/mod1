mod camera;
mod entities;
mod error;
mod input;
mod mesh;
mod render;
mod shader_program;

pub use camera::Camera;
pub use entities::{Entity, EntityStore};
pub use error::EngineError;
pub use input::{ClickButton, Inputs};
pub use mesh::Mesh;
pub use render::{Renderable, Renderer, RendererBuilder};
pub use shader_program::ShaderProgram;

use glutin::{
	event::{Event, WindowEvent},
	event_loop::{ControlFlow, EventLoopWindowTarget},
};
use std::time;

pub fn core_loop(
	mut renderer: Renderer,
	mut entities: EntityStore,
) -> Box<dyn FnMut(Event<'_, ()>, &EventLoopWindowTarget<()>, &mut ControlFlow)> {
	let mut inputs = Inputs::new();
	let mut last_draw = time::Instant::now();
	renderer.load_shader("triangle");
	let triangle = Mesh::new("triangle");
	// renderer.clear();
	// renderer.swap();
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
				_ => (),
			},
			Event::RedrawRequested(_) => {
				last_draw = time::Instant::now();
				renderer.clear();
				renderer.draw(&triangle, &mut entities);
				if let Err(err) = renderer.swap() {
					eprintln!("{}", err);
					logic_frame = false;
					*flow = ControlFlow::Exit;
				}
			}
			_ => {}
		}
		if logic_frame {
			entities.update(delta as f32 / 1000000.0, &inputs);
			entities.exec_clear();
			inputs.update();
		}
	})
}
