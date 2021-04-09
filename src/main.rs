mod engine;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use std::time;

fn main() {
    let events_loop = EventLoop::new();
    let mut renderer = engine::RendererBuilder::new()
        .title("mod1")
        .size((1280, 720))
        .resizable(true)
        .build(&events_loop);

    renderer.load_shader("triangle");
    let triangle = engine::Mesh::new("triangle");
    let mut last_draw = time::Instant::now();
    renderer.clear();
    renderer.swap();
    events_loop.run(move |event, _target, flow| {
        *flow = ControlFlow::Poll;
        let now = time::Instant::now();
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => renderer.resize(size),
                _ => (),
            },
            Event::RedrawRequested(_) => {}
            _ => {}
        }
        let elapsed = now.duration_since(last_draw).as_millis();
        if elapsed > 16 {
            println!("{}", 1000 / elapsed);
            renderer.clear();
            renderer.draw(&triangle);
            renderer.swap();
            last_draw = time::Instant::now();
        }
    });
}
