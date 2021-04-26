mod algo;
mod engine;

use algo::surfaces::HeightMap;
use engine::Mesh;

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
    let map: Vec<f32> = vec![
        0.0, 0.0, 0.0, 0.0, 0.0, // first row
        0.0, 1.0, 1.0, 1.0, 0.0, // second row
        0.0, 1.0, 1.5, 1.0, 0.0, // third row
        0.0, 1.0, 1.0, 1.0, 0.0, // fourth row
        0.0, 0.0, 0.0, 0.0, 0.0, // fifth row
    ];
    let terrain = Box::new(HeightMap::new(map, 5, 1.0));
    let vertices = terrain.gen_mesh_vertices();
    let terrain_mesh = Box::new(Mesh::new("terrain", &vertices, (5 - 1) as f32 * 0.5));
    entities.insert(terrain);
    entities.insert(terrain_mesh);

    event_loop.run(engine::core_loop(renderer, entities));
}
