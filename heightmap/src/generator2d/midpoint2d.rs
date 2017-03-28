//! Midpoint displacement algorithm
use rand::distributions::{Sample, Range};
use rand::Rng;

use std::cmp::max;

use generator2d::Generator2d;
use heightmap::{Heightmap, heightmap_from_vec};

pub struct Midpoint2d {}

impl Midpoint2d {
    pub fn new() -> Midpoint2d {
        Midpoint2d {}
    }
}

impl Generator2d for Midpoint2d {
    /// Generates a map using the midpoint displacement algorithm.
    ///
    /// The size doesn't have to be 2^n + 1
    fn generate<R>(&self, width: u32, height: u32, rng: &mut R) -> Heightmap
        where R: Rng
    {
        let mut sampler = Range::new(0.0, 1.0);

        let size = ((max(width, height) - 1).next_power_of_two() + 1) as usize;
        let mut data = vec![0.0; size*size];
        let idx = |x: usize, y: usize| y * size + x;

        data[idx(0, 0)] = sampler.sample(rng);
        data[idx(size - 1, 0)] = sampler.sample(rng);
        data[idx(0, size - 1)] = sampler.sample(rng);
        data[idx(size - 1, size - 1)] = sampler.sample(rng);

        let mut d = size - 1;

        while d > 1 {
            let d_2 = d >> 1;
            let delta = d as f64;
            let mut sampler = Range::new(-delta, delta);

            for x in (d_2..size).step_by(d) {
                for y in (d_2..size).step_by(d) {
                    let tl = data[idx(x - d_2, y - d_2)];
                    let tr = data[idx(x - d_2, y + d_2)];
                    let bl = data[idx(x + d_2, y - d_2)];
                    let br = data[idx(x + d_2, y + d_2)];

                    let center = (tl + tr + bl + br) / 4.0;
                    let top = (tl + tr) / 2.0;
                    let bottom = (bl + br) / 2.0;
                    let left = (tl + bl) / 2.0;
                    let right = (tr + br) / 2.0;

                    data[idx(x, y)] = center + sampler.sample(rng);
                    data[idx(x - d_2, y)] = top + sampler.sample(rng);
                    data[idx(x + d_2, y)] = bottom + sampler.sample(rng);
                    data[idx(x, y - d_2)] = left + sampler.sample(rng);
                    data[idx(x, y + d_2)] = right + sampler.sample(rng);
                }
            }

            d = d_2;
        }

        heightmap_from_vec(size as u32, size as u32, data).submap(0, 0, width, height)
    }
}
