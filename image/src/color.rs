use image::Pixel;
use image::Rgb;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::Error;
use std::fmt;
use std::ops::Deref;

#[derive(Clone, Copy, Debug)]
pub struct Color(Rgb<u8>);

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.collect_str(&format_args!("{} {} {}", self.red(), self.green(), self.blue()))
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "[{}, {}, {}]",
               self.0.data[0],
               self.0.data[1],
               self.0.data[2])
    }
}

impl Deserialize for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        let string = String::deserialize(deserializer)?;
        let mut iter = string.split_whitespace();

        let red = iter.next()
            .map(|s| s.parse().map_err(D::Error::custom))
            .unwrap_or_else(||Err(D::Error::custom("missing value")))?;
        let green = iter.next()
            .map(|s| s.parse().map_err(D::Error::custom))
            .unwrap_or_else(||Err(D::Error::custom("missing value")))?;
        let blue = iter.next()
            .map(|s| s.parse().map_err(D::Error::custom))
            .unwrap_or_else(||Err(D::Error::custom("missing value")))?;

        Ok(Color::new([red, green, blue]))
    }
}

impl Deref for Color {
    type Target = Rgb<u8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Color {
    pub fn new(rgb: [u8; 3]) -> Color {
        Color(Rgb(rgb))
    }

    pub fn red(self) -> u8 {
        self.0.data[0]
    }

    pub fn green(self) -> u8 {
        self.0.data[1]
    }

    pub fn blue(self) -> u8 {
        self.0.data[2]
    }
}

pub fn lerp(left: Color, right: Color, t: f64) -> Color {
    if t < 0.0 {
        left
    } else if t > 1.0 {
        right
    } else {
        let f = |l, r| {
            let (tl, tr): (u8, u8) = (l, r);
            ((1.0 - t) * (tl as f64) + t * (tr as f64)) as u8
        };
        Color(left.map2(&*right, f))
    }
}
