mod engine;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let mut renderer = match engine::RendererBuilder::new()
        .title("mod1")
        .size((1280, 720))
        .resizable(true)
        .build(&event_loop)
    {
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
        Ok(renderer) => renderer,
    };
    let mut entities = engine::EntityStore::new();
    let cam = Box::new(engine::Camera::new());
    let cam_key = entities.insert(cam);
    renderer.set_cam(Some(cam_key));
    event_loop.run(engine::core_loop(renderer, entities));
}
