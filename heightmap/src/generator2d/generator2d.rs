use rand::Rng;

use heightmap::Heightmap;

pub trait Generator2d {
    fn generate<R: Rng>(&self, width: u32, height: u32, rng: &mut R) -> Heightmap;
}
