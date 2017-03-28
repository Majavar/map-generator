// Source: http://webstaff.itn.liu.se/~stegu/simplexnoise/simplexnoise.pdf
use rand::Rng;

use noise2d::Noise2d;

pub struct Simplex2d {
    permutations: [u8; 256],
}

fn grad(hash: u8, x: f64, y: f64) -> f64 {
    match hash & 0b111 {
        0b000 => x + y,
        0b001 => x,
        0b010 => x - y,
        0b011 => y,
        0b100 => -y,
        0b101 => -x + y,
        0b110 => -x,
        0b111 => -x - y,
        _ => unreachable!(),
    }
}

impl Simplex2d {
    pub fn new<R: Rng>(r: &mut R) -> Simplex2d {
        let mut permutations = [0; 256];
        for (i, x) in permutations.iter_mut().enumerate() {
            *x = i as u8
        }
        r.shuffle(&mut permutations);

        Simplex2d { permutations: permutations }
    }

    fn idx(&self, x: usize, y: usize) -> u8 {
        let idx = y & 0xFF;
        let idx = (x + (self.permutations[idx] as usize)) & 0xFF;
        self.permutations[idx]
    }
}

impl Noise2d for Simplex2d {
    fn at(&self, x: f64, y: f64) -> f64 {
        let f2 = 0.366025403784438646763723170752;
        let g2 = 0.211324865405187117745425609748;

        let s = (x + y) * f2;
        let i = (x + s) as usize;
        let j = (y + s) as usize;

        let t = ((i + j) as f64) * g2;
        let x0 = x - (i as f64) + t;
        let y0 = y - (j as f64) + t;

        let (i1, j1) = if x0 > y0 { (1, 0) } else { (0, 1) };

        let x1 = x0 - (i1 as f64) + g2;
        let y1 = y0 - (j1 as f64) + g2;

        let v = g2.mul_add(2.0, -1.0);
        let x2 = x0 + v;
        let y2 = y0 + v;

        let t0 = 0.5 - x0 * x0 - y0 * y0;
        let t1 = 0.5 - x1 * x1 - y1 * y1;
        let t2 = 0.5 - x2 * x2 - y2 * y2;

        let n0 = if t0.is_sign_positive() {
            let gi0 = self.idx(i, j);
            let d0 = t0 * t0;
            d0 * d0 * grad(gi0, x0, y0)
        } else {
            0.0
        };

        let n1 = if t1.is_sign_positive() {
            let gi1 = self.idx(i + i1, j + j1);
            let d1 = t1 * t1;
            d1 * d1 * grad(gi1, x1, y1)
        } else {
            0.0
        };

        let n2 = if t2.is_sign_positive() {
            let gi2 = self.idx(i + 1, j + 1);
            let d2 = t2 * t2;
            d2 * d2 * grad(gi2, x2, y2)
        } else {
            0.0
        };

        35.0 * (n0 + n1 + n2) + 0.5
    }
}
