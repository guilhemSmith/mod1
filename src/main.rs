mod algo;
mod engine;

use glam::Vec3;

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
    renderer.load_shader("border");

    let terrain = Box::new(HeightMap::new(&file_arg)?);
    let terrain_vert =
        Mesh::heights_gen_vertices(algo::DIM, &Vec::from(terrain.height_points().clone()));
    let terrain_mesh = Box::new(Mesh::new("terrain", &terrain_vert, algo::DIM, true, true));
    let border_vert = Mesh::wall_gen_vertices(&terrain.border_wall().clone());
    let border_mesh = Box::new(Mesh::new("border", &border_vert, algo::DIM, true, true));
    let terrain_id = entities.insert(terrain);
    entities.insert(terrain_mesh);
    entities.insert(border_mesh);

    renderer.load_shader("water");
    let water_vert = Mesh::heights_gen_vertices(algo::DIM, &vec![-0.1; algo::DIM * algo::DIM]);
    let water_mesh = Box::new(Mesh::new("water", &water_vert, algo::DIM, false, false));
    let water_id = entities.insert(water_mesh);
    let border_vert = Mesh::wall_gen_vertices(&vec![Vec3::ZERO; 800]);
    let border_mesh = Box::new(Mesh::new("water", &border_vert, algo::DIM, false, false));
    let border_id = entities.insert(border_mesh);
    let water = algo::Water::new(water_id, terrain_id, border_id);
    entities.insert(Box::new(water));
    let proxy = event_loop.create_proxy();
    event_loop.run(engine::core_loop(renderer, entities, proxy));
}
