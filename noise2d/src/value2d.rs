use rand::Rng;
use rand::distributions::{Range, Sample};
use std::ops::Fn;

use noise2d::Noise2d;

pub struct Value2d<F> {
    permutations: [u8; 256],
    values: [f64; 256],
    interpolate: F,
}

impl<F> Value2d<F>
    where F: Fn(f64, f64, f64) -> f64
{
    pub fn new<R: Rng>(r: &mut R, interpolate: F) -> Value2d<F> {
        let mut permutations = [0; 256];
        for (i, x) in permutations.iter_mut().enumerate() {
            *x = i as u8
        }
        r.shuffle(&mut permutations);

        let mut sampler = Range::new(0.0, 1.0);
        let mut values = [0.0; 256];
        for v in values.iter_mut() {
            *v = sampler.sample(r);
        }

        Value2d {
            permutations: permutations,
            values: values,
            interpolate: interpolate,
        }
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        let idx = y & 0xFF;
        let idx = (x + (self.permutations[idx] as usize)) & 0xFF;
        self.permutations[idx] as usize
    }
}

impl<F> Noise2d for Value2d<F>
    where F: Fn(f64, f64, f64) -> f64
{
    fn at(&self, x: f64, y: f64) -> f64 {
        let xint = x as usize;
        let yint = y as usize;

        let nw = self.values[self.idx(xint, yint)];
        let ne = self.values[self.idx(xint + 1, yint)];
        let sw = self.values[self.idx(xint, yint + 1)];
        let se = self.values[self.idx(xint + 1, yint + 1)];

        let xfract = x.fract();
        let yfract = y.fract();
        let n = (self.interpolate)(nw, ne, xfract);
        let s = (self.interpolate)(sw, se, xfract);

        (self.interpolate)(n, s, yfract)
    }
}
