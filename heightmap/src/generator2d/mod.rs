mod diamond2d;
mod fractal2d;
mod midpoint2d;

pub use self::diamond2d::Diamond2d;
pub use self::fractal2d::Fractal2d;
pub use self::midpoint2d::Midpoint2d;

use rand::Rng;
use heightmap::Heightmap;

pub trait Generator2d {
    fn generate<R: Rng>(&self, width: u32, height: u32, rng: &mut R) -> Heightmap;
}
