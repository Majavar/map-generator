use image::{Color, ColorRamp, Vec3};
use rand::{Rng, StdRng};
use serde::{de, ser};
use std::convert::From;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::str::FromStr;
use std::path::Path;
use serde_yaml;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Generator {
    Diamond,
    Fractal,
    Midpoint,
}

impl Generator {
    const VARIANTS: &'static [&'static str] = &["diamond", "fractal", "midpoint"];
}

impl FromStr for Generator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "diamond" => Ok(Generator::Diamond),
            "fractal" => Ok(Generator::Fractal),
            "midpoint" => Ok(Generator::Midpoint),
            s => Err(format!("Cannot convert {} to Generator", s)),
        }
    }
}

impl From<Generator> for &'static str {
    fn from(generator: Generator) -> Self {
        match generator {
            Generator::Diamond => Generator::VARIANTS[0],
            Generator::Fractal => Generator::VARIANTS[1],
            Generator::Midpoint => Generator::VARIANTS[2],
        }
    }
}

impl de::Deserialize for Generator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: de::Deserializer
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| de::Error::unknown_variant(&s, Generator::VARIANTS))
    }
}

impl ser::Serialize for Generator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ser::Serializer
    {
        serializer.serialize_str(From::from(*self))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Interpolation {
    Linear,
    Cubic,
    Quintic,
    Cosine,
}

impl Interpolation {
    const VARIANTS: &'static [&'static str] = &["linear", "cubic", "quintic", "cosine"];
}

impl FromStr for Interpolation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "linear" => Ok(Interpolation::Linear),
            "cubic" => Ok(Interpolation::Cubic),
            "quintic" => Ok(Interpolation::Quintic),
            "cosine" => Ok(Interpolation::Cosine),
            s => Err(format!("Cannot convert {} to Interpolation", s)),
        }
    }
}

impl From<Interpolation> for &'static str {
    fn from(interpolation: Interpolation) -> Self {
        match interpolation {
            Interpolation::Linear => Interpolation::VARIANTS[0],
            Interpolation::Cubic => Interpolation::VARIANTS[1],
            Interpolation::Quintic => Interpolation::VARIANTS[2],
            Interpolation::Cosine => Interpolation::VARIANTS[3],
        }
    }
}

impl de::Deserialize for Interpolation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: de::Deserializer
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| de::Error::unknown_variant(&s, Interpolation::VARIANTS))
    }
}

impl ser::Serialize for Interpolation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ser::Serializer
    {
        serializer.serialize_str(From::from(*self))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Noise {
    Value,
    Gradient,
    Simplex,
}

impl Noise {
    const VARIANTS: &'static [&'static str] = &["value", "gradient", "simplex"];
}

impl FromStr for Noise {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "value" => Ok(Noise::Value),
            "gradient" => Ok(Noise::Gradient),
            "simplex" => Ok(Noise::Simplex),
            s => Err(format!("Cannot convert {} to Noise", s)),
        }
    }
}

impl From<Noise> for &'static str {
    fn from(noise: Noise) -> Self {
        match noise {
            Noise::Value => Noise::VARIANTS[0],
            Noise::Gradient => Noise::VARIANTS[1],
            Noise::Simplex => Noise::VARIANTS[2],
        }
    }
}

impl de::Deserialize for Noise {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: de::Deserializer
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| de::Error::unknown_variant(&s, Noise::VARIANTS))
    }
}

impl ser::Serialize for Noise {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ser::Serializer
    {
        serializer.serialize_str(From::from(*self))
    }
}

#[derive(Builder, Debug, Deserialize, Serialize)]
pub struct MapGeneratorConfig {
    generator: Generator,
    noise: Option<Noise>,
    scale: Option<f64>,
    octave: Option<u32>,
    lacunarity: Option<f64>,
    persistance: Option<f64>,
    interpolation: Option<Interpolation>,
    #[serde(default = "default_ramp")]
    ramp: ColorRamp,
    #[serde(default = "default_light_position")]
    light_position: Vec3,
    #[serde(default = "default_light")]
    light: Color,
    #[serde(default = "default_dark")]
    dark: Color,
    #[serde(default = "default_output")]
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    output: String,
    #[serde(default = "default_seed")]
    seed: usize,
}

pub fn default_output() -> String {
    "out.png".to_string()
}

pub fn default_seed() -> usize {
    StdRng::new().unwrap().gen()
}

pub fn default_light_position() -> Vec3 {
    Vec3::new(-1.0, -1.0, 0.0)
}

pub fn default_light() -> Color {
    Color::new([0xFFu8, 0xFFu8, 0xCCu8])
}

pub fn default_dark() -> Color {
    Color::new([0x33u8, 0x11u8, 0x33u8])
}

pub fn default_ramp() -> ColorRamp {
    let mut ramp = ColorRamp::new();

    ramp.add_step(0.000, Color::new([2u8, 43u8, 68u8])); // very dark blue: deep water
    ramp.add_step(0.250, Color::new([9u8, 62u8, 92u8])); // dark blue: water
    ramp.add_step(0.490, Color::new([17u8, 82u8, 112u8])); // blue: shallow water
    ramp.add_step(0.500, Color::new([69u8, 108u8, 118u8])); // light blue: shore
    ramp.add_step(0.510, Color::new([42u8, 102u8, 41u8])); // green: grass
    ramp.add_step(0.750, Color::new([115u8, 128u8, 77u8])); // light green: veld
    ramp.add_step(0.850, Color::new([153u8, 143u8, 92u8])); // brown: tundra
    ramp.add_step(0.950, Color::new([179u8, 179u8, 179u8])); // grey: rocks
    ramp.add_step(1.000, Color::new([255u8, 255u8, 255u8])); // white: snow

    ramp
}

impl MapGeneratorConfig {
    pub fn read<P: AsRef<Path>>(path: P) -> io::Result<MapGeneratorConfig> {
        let mut file = File::open(path)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        serde_yaml::from_str(contents.as_str()).map_err(|e| io::Error::new(io::ErrorKind::Other, e.description()))
    }

    pub fn write<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let mut buffer = File::create(path)?;

        let contents = serde_yaml::to_string(self).map_err(|e| io::Error::new(io::ErrorKind::Other, e.description()))?;
        buffer.write_all(contents.as_bytes())
    }

    pub fn generator(&self) -> &Generator {
        &self.generator
    }

    pub fn noise(&self) -> &Option<Noise> {
        &self.noise
    }

    pub fn scale(&self) -> &Option<f64> {
        &self.scale
    }

    pub fn octave(&self) -> &Option<u32> {
        &self.octave
    }

    pub fn lacunarity(&self) -> &Option<f64> {
        &self.lacunarity
    }

    pub fn persistance(&self) -> &Option<f64> {
        &self.persistance
    }

    pub fn interpolation(&self) -> &Option<Interpolation> {
        &self.interpolation
    }

    pub fn light_position(&self) -> &Vec3 {
        &self.light_position
    }

    pub fn light(&self) -> &Color {
        &self.light
    }

    pub fn dark(&self) -> &Color {
        &self.dark
    }

    pub fn output(&self) -> &String {
        &self.output
    }

    pub fn set_output(mut self, output: Option<String>) -> Self {
        if let Some(o) = output {
            self.output = o;
        }

        self
    }

    pub fn seed(&self) -> &usize {
        &self.seed
    }

    pub fn set_seed(mut self, seed: Option<usize>) -> Self {
        if let Some(s) = seed {
            self.seed = s;
        }

        self
    }

    pub fn ramp(&self) -> &ColorRamp {
        &self.ramp
    }
}
