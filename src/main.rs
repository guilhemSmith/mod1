mod algo;
mod engine;

use algo::HeightMap;
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

    renderer.load_shader("terrain");
    let map = vec![
        Vec3::new(40.0, 40.0, 30.0),
        Vec3::new(40.0, 50.0, 30.0),
        Vec3::new(40.0, 60.0, 30.0),
        Vec3::new(50.0, 60.0, 30.0),
        Vec3::new(50.0, 50.0, 10.0),
        Vec3::new(50.0, 40.0, 30.0),
        Vec3::new(60.0, 40.0, 30.0),
        Vec3::new(60.0, 50.0, 30.0),
        Vec3::new(60.0, 60.0, 30.0),
    ];
    let terrain = Box::new(HeightMap::new(map, 1.0));
    let vertices = terrain.gen_mesh_vertices();
    let terrain_mesh = Box::new(Mesh::new(
        "terrain",
        &vertices,
        (100 - 1) as f32 * 0.5 * 1.0,
    ));
    entities.insert(terrain);
    entities.insert(terrain_mesh);

    event_loop.run(engine::core_loop(renderer, entities));
}
