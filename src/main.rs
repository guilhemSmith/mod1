mod engine;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};

fn main() {
    let events_loop = EventLoop::new();
    let renderer = engine::RendererBuilder::new()
        .title("mod1")
        .size((1280, 720))
        .resizable(false)
        .build(&events_loop);

    let (shader_program, vao) = engine::shader_program::compile("triangle");

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
