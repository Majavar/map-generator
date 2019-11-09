use color::{Color, lerp};
use heightmap::Heightmap;
use image::{ImageBuffer, Rgb};
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::Error;

#[derive(Clone, Copy, Debug)]
pub struct Vec3(f64, f64, f64);

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3(x, y, z)
    }

    fn dot(self, v: &Vec3) -> f64 {
        self.0 * v.0 + self.1 * v.1 + self.2 * v.2
    }
}

impl Serialize for Vec3 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.collect_str(&format_args!("{} {} {}", self.0, self.1, self.2))
    }
}

impl Deserialize for Vec3 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        let string = String::deserialize(deserializer)?;
        let mut iter = string.split_whitespace();

        let x = iter.next()
            .map(|s| s.parse().map_err(D::Error::custom))
            .unwrap_or_else(||Err(D::Error::custom("missing value")))?;
        let y = iter.next()
            .map(|s| s.parse().map_err(D::Error::custom))
            .unwrap_or_else(||Err(D::Error::custom("missing value")))?;
        let z = iter.next()
            .map(|s| s.parse().map_err(D::Error::custom))
            .unwrap_or_else(||Err(D::Error::custom("missing value")))?;

        Ok(Vec3::new(x, y, z))
    }
}

trait Normal {
    fn normal(&self, x: u32, y: u32) -> Vec3;
}

impl Normal for Heightmap {
    fn normal(&self, x: u32, y: u32) -> Vec3 {
        let nx = if x == 0 {
            (self.get(x + 1, y) - self.get(x, y)) * 2.0
        } else if x == self.width() - 1 {
            (self.get(x, y) - self.get(x - 1, y)) * 2.0
        } else {
            (self.get(x + 1, y) - self.get(x - 1, y))
        };

        let ny = if y == 0 {
            (self.get(x, y + 1) - self.get(x, y)) * 2.0
        } else if y == self.height() - 1 {
            (self.get(x, y) - self.get(x, y - 1)) * 2.0
        } else {
            (self.get(x, y + 1) - self.get(x, y - 1))
        };

        let n = (nx * nx + ny * ny + 4.0).sqrt();

        Vec3(-nx / n, -ny / n, 2.0 / n)
    }
}

pub trait Shadable {
    fn shade(&mut self, hmap: &Heightmap, light: &Vec3, c0: Color, c1: Color);
}

impl Shadable for ImageBuffer<Rgb<u8>, Vec<u8>> {
    fn shade(&mut self, hmap: &Heightmap, light: &Vec3, c0: Color, c1: Color) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                if hmap.get(x, y) > 0.5 {
                    let mut d = light.dot(&hmap.normal(x, y));
                    d = d * 35.0 + 0.5;
                    let p = self.get_pixel_mut(x, y);

                    *p = if d < 0.0 {
                        *c1
                    } else if d > 1.0 {
                        *c0
                    } else if d < 0.5 {
                        *lerp(c1, Color::new(p.data), 2.0 * d)
                    } else {
                        *lerp(Color::new(p.data), c0, 2.0 * d - 1.0)
                    };
                }
            }
        }
    }
}
