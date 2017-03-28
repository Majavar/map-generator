use std::path::Path;
use rand::{Rng, StdRng, SeedableRng};

use config::{Generator, Interpolation, MapGeneratorConfig, Noise};
use heightmap::{Diamond2d, Fractal2d, Generator2d, Midpoint2d};
use image::Shadable;
use interpolate;
use noise2d::{Gradient2d, Simplex2d, Value2d};

pub struct MapGenerator {
    config: MapGeneratorConfig,
}

impl MapGenerator {
    pub fn new(config: MapGeneratorConfig) -> MapGenerator {
        MapGenerator { config: config }
    }

    pub fn run(&self) {
        let seed: &[_] = &[*self.config.seed()];
        let mut r: StdRng = SeedableRng::from_seed(seed);

        match (self.config.generator(), self.config.noise().unwrap_or(Noise::Gradient)) {
            (&Generator::Diamond, _) => self.generate(Diamond2d::new(), &mut r),
            (&Generator::Fractal, Noise::Value) => {
                self.generate(Fractal2d::new(Value2d::new(&mut r,
                                                          interpolate::get(self.config
                                                                               .interpolation()
                                                                               .unwrap_or(Interpolation::Cubic))),
                                             self.config.scale().unwrap_or(2.0),
                                             self.config.octave().unwrap_or(10),
                                             self.config.lacunarity().unwrap_or(2.0),
                                             self.config.persistance().unwrap_or(0.5)),
                              &mut r);
            }
            (&Generator::Fractal, Noise::Gradient) => {
                self.generate(
                    Fractal2d::new(
                        Gradient2d::new(&mut r, interpolate::get(self.config.interpolation().unwrap_or(Interpolation::Cubic))),
                        self.config.scale().unwrap_or(2.0),
                        self.config.octave().unwrap_or(10),
                        self.config.lacunarity().unwrap_or(2.0),
                        self.config.persistance().unwrap_or(0.5)
                    ),
                    &mut r
                )
            }
            (&Generator::Fractal, Noise::Simplex) => {
                self.generate(Fractal2d::new(Simplex2d::new(&mut r),
                                             self.config.scale().unwrap_or(2.0),
                                             self.config.octave().unwrap_or(10),
                                             self.config.lacunarity().unwrap_or(2.0),
                                             self.config.persistance().unwrap_or(0.5)),
                              &mut r)
            }
            (&Generator::Midpoint, _) => self.generate(Midpoint2d::new(), &mut r),
        };
    }

    fn generate<G, R>(&self, g: G, rng: &mut R)
        where G: Generator2d,
              R: Rng
    {
        let file = &Path::new(self.config.output());
        let mut hmap = g.generate(*self.config.width(), *self.config.height(), rng);
        hmap.normalize();
        hmap.flatten();

        let mut img = self.config.ramp().apply_on(&hmap);
        img.shade(&hmap,
                  self.config.light_position(),
                  self.config.light(),
                  self.config.dark());

        let _ = img.save(file);
    }
}
