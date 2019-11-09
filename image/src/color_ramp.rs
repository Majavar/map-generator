//! Associate a `Color` to a value.
use color::{Color, lerp};
use heightmap::Heightmap;
use image::{ImageBuffer, Rgb};
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::Error;
use std::vec::Vec;

#[derive(Clone, Copy, Debug)]
struct ColorStep {
    value: f64,
    color: Color,
}

impl Serialize for ColorStep {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.collect_str(&format_args!("{:.8} {: >3} {: >3} {: >3}",
                                             self.value,
                                             self.color.red(),
                                             self.color.green(),
                                             self.color.blue()))
    }
}

impl Deserialize for ColorStep {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        let string = String::deserialize(deserializer)?;
        let mut iter = string.split_whitespace();


        let value = iter.next()
            .map(|s| s.parse().map_err(D::Error::custom))
            .unwrap_or_else(||Err(D::Error::custom("missing value")))?;

        let red = iter.next()
            .map(|s| s.parse().map_err(D::Error::custom))
            .unwrap_or_else(||Err(D::Error::custom("missing value")))?;
        let green = iter.next()
            .map(|s| s.parse().map_err(D::Error::custom))
            .unwrap_or_else(||Err(D::Error::custom("missing value")))?;
        let blue = iter.next()
            .map(|s| s.parse().map_err(D::Error::custom))
            .unwrap_or_else(||Err(D::Error::custom("missing value")))?;

        Ok(ColorStep {
               value,
               color: Color::new([red, green, blue]),
           })
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, Default)]
pub struct ColorRamp {
    colors: Vec<ColorStep>,
}

impl ColorRamp {
    /// Create an empty Color.
    pub fn new() -> ColorRamp {
        Default::default()
    }

    pub fn add_step(&mut self, step: f64, color: Color) {
        let s = ColorStep {
            value: step,
            color,
        };

        match self.colors.iter().position(|ref x| x.value >= s.value) {
            Some(i) => self.colors.insert(i, s),
            None => self.colors.push(s),
        }
    }

    pub fn get(&self, pos: f64) -> Color {
        match self.colors.iter().position(|ref x| x.value >= pos) {
            None => {
                self.colors
                    .last()
                    .unwrap()
                    .color
            }
            Some(0) => self.colors[0].color,
            Some(x) => {
                let s1 = &self.colors[x - 1];
                let s2 = &self.colors[x];

                let t = (pos - s1.value) / (s2.value - s1.value);
                lerp(s1.color, s2.color, t)
            }
        }
    }

    pub fn apply_on(&self, map: &Heightmap) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        ImageBuffer::from_fn(map.width(), map.height(), |x, y| *self.get(map.get(x, y)))
    }
}
