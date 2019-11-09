use rand::Rng;

use super::Generator2d;
use heightmap::{Heightmap, heightmap_from_iter};
use noise2d::Noise2d;

pub struct Fractal2d<N> {
    noise: N,
    scale: f64,
    octave: u32,
    lacunarity: f64,
    persistance: f64,
}

impl<N> Fractal2d<N>
    where N: Noise2d
{
    pub fn new(noise: N, scale: f64, octave: u32, lacunarity: f64, persistance: f64) -> Fractal2d<N> {
        Fractal2d {
            noise,
            scale,
            octave,
            lacunarity,
            persistance,
        }
    }

    fn get(&self, x: f64, y: f64) -> f64 {
        let mut value = 0.0;
        let mut frequency = 1.0;
        let mut amplitude = 1.0;

        for _ in 0..self.octave {
            value +=  self.noise.at(x * frequency, y * frequency) * amplitude;

            frequency *=  self.lacunarity;
            amplitude *=  self.persistance;
        }

        value
    }
}

impl<N> Generator2d for Fractal2d<N>
    where N: Noise2d
{
    fn generate<R: Rng>(&self, width: u32, height: u32, _: &mut R) -> Heightmap {
        let wt = width as f64;
        let ht = height as f64;
        let ratio = wt / ht;
        let f = |x: u32| ((x % width) as f64) / wt * self.scale * ratio;
        let g = |y: u32| ((y / width) as f64) / ht * self.scale;

        heightmap_from_iter(width,
                            height,
                            (0..width * height).map(|i| self.get(f(i), g(i))))
    }
}
