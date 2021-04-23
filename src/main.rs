mod engine;

use engine::Mesh;
use glam::Vec3;

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
    let cam = Box::new(engine::Camera::new(true));
    let cam_key = entities.insert(cam);
    renderer.set_cam(Some(cam_key));

    renderer.load_shader("triangle");
    let vertices = vec![
        Vec3::new(-1.0, 0.0, -1.0),
        Vec3::new(1.0, 0.0, -1.0),
        Vec3::new(1.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 1.0),
        Vec3::new(-1.0, 0.0, 1.0),
        Vec3::new(-1.0, 0.0, -1.0),
        Vec3::new(1.0, 0.0, 1.0),
        Vec3::new(-1.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 3.0),
        Vec3::new(1.0, 0.0, 3.0),
        Vec3::new(-1.0, 0.0, 3.0),
        Vec3::new(-1.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 1.0),
        Vec3::new(-1.0, 0.0, -1.0),
        Vec3::new(1.0, 0.0, -3.0),
        Vec3::new(1.0, 0.0, -3.0),
        Vec3::new(-1.0, 0.0, -3.0),
        Vec3::new(-1.0, 0.0, -1.0),
    ];
    let triangle = Box::new(Mesh::new("triangle", &vertices));
    entities.insert(triangle);

    event_loop.run(engine::core_loop(renderer, entities));
}
