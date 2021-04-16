mod engine;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let renderer = engine::RendererBuilder::new()
        .title("mod1")
        .size((1280, 720))
        .resizable(true)
        .build(&event_loop);
    let entities = engine::EntityStore::new();
    event_loop.run(engine::core_loop(renderer, entities));
}
