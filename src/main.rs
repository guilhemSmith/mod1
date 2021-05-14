mod algo;
mod engine;

use algo::HeightMap;
use engine::{Camera, EntityStore, Mesh, PolygonMode};

fn main() {
    match exec_main() {
        Err(err) => eprintln!("{}", err),
        Ok(()) => {}
    }
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
        .resizable(true)
        .build(&event_loop)?;
    let mut entities = EntityStore::new();
    let cam = Box::new(Camera::new(true, Some(PolygonMode::Face)));
    let cam_key = entities.insert(cam);
    renderer.set_cam(Some(cam_key));

    renderer.load_shader("terrain");

    // let file_path =
    let terrain = Box::new(HeightMap::new(&file_arg)?);
    let terrain_mesh = Box::new(Mesh::new(
        "terrain",
        &Vec::from(terrain.height_points().clone()),
        algo::DIM,
        true,
        true,
    ));
    let terrain_id = entities.insert(terrain);
    entities.insert(terrain_mesh);

    renderer.load_shader("water");
    let water_mesh = Box::new(Mesh::new(
        "water",
        &vec![-0.1; algo::DIM * algo::DIM],
        algo::DIM,
        false,
        false,
    ));
    let water_id = entities.insert(water_mesh);
    let water = algo::Water::new(water_id, terrain_id);
    entities.insert(Box::new(water));
    let proxy = event_loop.create_proxy();
    event_loop.run(engine::core_loop(renderer, entities, proxy));
}
