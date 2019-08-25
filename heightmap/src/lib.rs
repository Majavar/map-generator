extern crate noise2d;
extern crate rand;

mod generator2d;
mod heightmap;

pub use heightmap::Heightmap;
pub use generator2d::{Generator2d, Diamond2d, Fractal2d, Midpoint2d};
