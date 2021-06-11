mod algo;
mod engine;

use algo::{HeightMap, Rain, Water};
use engine::{Camera, EntityStore, PolygonMode, Renderer};

fn main() {
    match exec_main() {
        Err(err) => eprintln!("{}", err),
        Ok(()) => {}
    }
}

fn load_shaders(renderer: &mut Renderer) {
    renderer.load_shader("terrain", true);
    renderer.load_shader("border", true);
    renderer.load_shader("water", true);
    renderer.load_shader("rain", false);
    renderer.load_shader("sun", false);
}

fn exec_main() -> Result<(), Box<dyn std::error::Error>> {
    let file_arg = std::env::args()
        .skip(1)
        .next()
        .ok_or(String::from("Not enough argument, need 1 file path."))?;

    let event_loop = glutin::event_loop::EventLoop::new();
    let mut renderer = engine::RendererBuilder::new()
        .title("mod1")
        .size((1280, 720))
        .resizable(false)
        .build(&event_loop)?;
    let mut entities = EntityStore::new();

    let cam = Box::new(Camera::new(true, Some(PolygonMode::Face)));
    let cam_key = entities.insert(cam);
    renderer.set_cam(Some(cam_key));
    load_shaders(&mut renderer);

    let light = engine::MeshPoints::new(
        "sun",
        &engine::MeshPoints::points_vertices(&vec![glam::Vec3::new(50.0, 50.0, 50.0)]),
        algo::DIM,
        true,
        true,
    );
    entities.insert(Box::new(light));

    let terrain = Box::new(HeightMap::new(&file_arg)?);
    let terrain_id = entities.insert(terrain);

    let water = Water::new(&entities, terrain_id);
    let water_id = entities.insert(Box::new(water));

    let rain = Rain::new(&entities, water_id);
    entities.insert(Box::new(rain));

    let proxy = event_loop.create_proxy();
    event_loop.run(engine::core_loop(renderer, entities, proxy));
}
