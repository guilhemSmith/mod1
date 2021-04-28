mod heightmap;
mod water;

pub use heightmap::HeightMap;
pub use water::Water;

pub const DIM: usize = 100;

pub type Map<const SIZE: usize> = [f32; SIZE];
