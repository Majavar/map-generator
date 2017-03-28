//! Midpoint displacement algorithm
use rand::distributions::{Sample, Range};
use rand::Rng;

use std::cmp::max;

use generator2d::Generator2d;
use heightmap::{Heightmap, heightmap_from_vec};

pub struct Diamond2d {}

pub fn square<R>(data: &mut Vec<f64>, x: usize, y: usize, size: usize, d: usize, sampler: &mut Range<f64>, rng: &mut R)
    where R: Rng
{
    let idx = |x: usize, y: usize| y * size + x;

    let tl = data[idx(x - d, y - d)];
    let tr = data[idx(x - d, y + d)];
    let bl = data[idx(x + d, y - d)];
    let br = data[idx(x + d, y + d)];

    let center = (tl + tr + bl + br) / 4.0;

    data[idx(x, y)] = center + sampler.sample(rng);
}

pub fn diamond<R>(data: &mut Vec<f64>,
                  x: usize,
                  y: usize,
                  size: usize,
                  d: usize,
                  sampler: &mut Range<f64>,
                  rng: &mut R)
    where R: Rng
{
    let mut sum = 0.0;
    let mut count = 0.0;
    let idx = |x: usize, y: usize| y * size + x;

    if x > 0 {
        sum = sum + data[idx(x - d, y)];
        count += 1.0;
    }
    if x < size - 1 {
        sum = sum + data[idx(x + d, y)];
        count += 1.0;
    }
    if y > 0 {
        sum = sum + data[idx(x, y - d)];
        count += 1.0;
    }
    if y < size - 1 {
        sum = sum + data[idx(x, y + d)];
        count += 1.0;
    }

    let val = sum / count;
    data[idx(x, y)] = val + sampler.sample(rng);
}

impl Diamond2d {
    pub fn new() -> Diamond2d {
        Diamond2d {}
    }
}

impl Generator2d for Diamond2d {
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
                    square(&mut data, x, y, size, d_2, &mut sampler, rng);
                }
            }

            for x in (d_2..size).step_by(d) {
                for y in (d_2..size).step_by(d) {
                    diamond(&mut data, x - d_2, y, size, d_2, &mut sampler, rng);
                    diamond(&mut data, x + d_2, y, size, d_2, &mut sampler, rng);
                    diamond(&mut data, x, y - d_2, size, d_2, &mut sampler, rng);
                    diamond(&mut data, x, y + d_2, size, d_2, &mut sampler, rng);
                }
            }

            d = d_2;
        }

        heightmap_from_vec(size as u32, size as u32, data).submap(0, 0, width, height)
    }
}
