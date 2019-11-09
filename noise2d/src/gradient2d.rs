use rand::Rng;
use rand::distributions::{Range, Sample};
use std::ops::Fn;

use noise2d::Noise2d;

#[derive(Copy, Clone)]
struct Vector2 {
    x: f64,
    y: f64,
}

pub struct Gradient2d<F> {
    permutations: [u8; 256],
    gradients: [Vector2; 256],
    interpolate: F,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Vector2 {
        Vector2 { x, y }
    }

    pub fn zero() -> Vector2 {
        Self::new(0.0, 0.0)
    }

    pub fn dot(self, v: &Vector2) -> f64 {
        self.x * v.x + self.y * v.y
    }
}

impl<F> Gradient2d<F>
    where F: Fn(f64, f64, f64) -> f64
{
    pub fn new<R: Rng>(r: &mut R, interpolate: F) -> Gradient2d<F> {
        let mut permutations = [0; 256];
        for (i, x) in permutations.iter_mut().enumerate() {
            *x = i as u8
        }
        r.shuffle(&mut permutations);

        let mut sampler = Range::new(0.0, 2.0 * ::std::f64::consts::PI);
        let mut gradients = [Vector2::zero(); 256];
        for g in gradients.iter_mut() {
            let (s, c) = sampler.sample(r).sin_cos();
            *g = Vector2::new(c, s);
        }

        Gradient2d {
            permutations,
            gradients,
            interpolate,
        }
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        let idx = y & 0xFF;
        let idx = (x + (self.permutations[idx] as usize)) & 0xFF;
        self.permutations[idx] as usize
    }
}

impl<F> Noise2d for Gradient2d<F>
    where F: Fn(f64, f64, f64) -> f64
{
    fn at(&self, x: f64, y: f64) -> f64 {
        let xi = x as usize;
        let yi = y as usize;

        let xf = x.fract();
        let yf = y.fract();

        let nw = self.gradients[self.idx(xi, yi)].dot(&Vector2::new(xf, yf));
        let ne = self.gradients[self.idx(xi + 1, yi)].dot(&Vector2::new(xf - 1.0, yf));
        let sw = self.gradients[self.idx(xi, yi + 1)].dot(&Vector2::new(xf, yf - 1.0));
        let se = self.gradients[self.idx(xi + 1, yi + 1)].dot(&Vector2::new(xf - 1.0, yf - 1.0));

        let n = (self.interpolate)(nw, ne, xf);
        let s = (self.interpolate)(sw, se, xf);

        (self.interpolate)(n, s, yf) / ::std::f64::consts::SQRT_2 + 0.5
    }
}
