mod render;

use glutin::dpi::PhysicalSize;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn main() {
    let events_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Glutin Triangle")
        .with_inner_size(PhysicalSize {
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
        });
    let gl_window = ContextBuilder::new()
        .with_vsync(true)
        .build_windowed(window, &events_loop)
        .unwrap();

    let gl_window = unsafe { gl_window.make_current().unwrap() };
    gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

    let (shader_program, vao) = render::shader_program::compile("triangle");

    events_loop.run(move |event, _target, flow| {
        *flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => render::resize(&gl_window, size),
                _ => (),
            },
            Event::RedrawRequested(_) => render::clear(&gl_window, shader_program, vao),
            _ => {}
        }
    });
}
