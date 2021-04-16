mod input;
mod mesh;
mod render;
mod shader_program;

pub use input::Inputs;
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
) -> Box<dyn FnMut(Event<'_, ()>, &EventLoopWindowTarget<()>, &mut ControlFlow)> {
	let mut inputs = Inputs::new();
	let mut last_draw = time::Instant::now();
	renderer.load_shader("triangle");
	let triangle = Mesh::new("triangle");
	renderer.clear();
	renderer.swap();
	Box::new(move |event, _target, flow: &mut ControlFlow| {
		*flow = ControlFlow::Poll;

		let delta = time::Instant::now().duration_since(last_draw).as_micros();
		if delta >= 8333 {
			renderer.window().request_redraw();
			let _fps = 1000000 / delta;
			// println!("{}", _fps);
		}

		inputs.update();
		match event {
			Event::LoopDestroyed => return,
			Event::WindowEvent { event, .. } => match event {
				WindowEvent::CloseRequested => *flow = ControlFlow::Exit,
				WindowEvent::Resized(size) => renderer.resize(size),
				WindowEvent::KeyboardInput { input, .. } => inputs.store_key(input),
				_ => (),
			},
			Event::RedrawRequested(_) => {
				renderer.clear();
				renderer.draw(&triangle);
				renderer.swap();
				last_draw = time::Instant::now();
			}
			_ => {}
		}
	})
}
