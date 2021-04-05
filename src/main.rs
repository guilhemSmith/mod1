mod render;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};

fn main() {
    let events_loop = EventLoop::new();
    let renderer = render::Renderer::new(&events_loop);

    let (shader_program, vao) = render::shader_program::compile("triangle");

    events_loop.run(move |event, _target, flow| {
        *flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => renderer.resize(size),
                _ => (),
            },
            Event::RedrawRequested(_) => {
                renderer.clear();
                renderer.draw(shader_program, vao);
                renderer.swap();
            }
            _ => {}
        }
    });
}
