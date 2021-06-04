mod heightmap;
mod rain;
mod water;

pub use heightmap::HeightMap;
pub use rain::Rain;
pub use water::Water;

const G: f32 = 9.81;
pub const DIM: usize = 100;

pub type Map<const SIZE: usize> = [f32; SIZE];
