#![feature(associated_consts)]
#![feature(proc_macro)]

extern crate clap;
#[macro_use]
extern crate derive_builder;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

extern crate heightmap;
extern crate noise2d;
extern crate image;

mod config;
mod interpolate;
mod map_generator;

use clap::{App, Arg, ArgGroup};
use rand::{Rng, StdRng};
use std::error::Error;
use std::path::Path;
use std::str::FromStr;

fn main() {
    let matches = App::new("Map Generator")
        .version("0.1")
        .author("Nico")
        .arg(Arg::with_name("config")
                 .short("c")
                 .long("config")
                 .help("config file")
                 .takes_value(true)
                 .value_name("FILE")
                 .validator(|c| if Path::new(&c).is_file() {
                                Ok(())
                            } else {
                                Err(format!("{} is not a valid filename", c))
                            }))
        .arg(Arg::with_name("seed")
                 .short("s")
                 .long("seed")
                 .help("seed used to produce randomness")
                 .takes_value(true)
                 .validator(|s| usize::from_str(&s).map(|_| ()).map_err(|e| e.description().to_string())))
        .arg(Arg::with_name("random-seed")
                 .short("r")
                 .long("random-seed")
                 .help("Use a random seed")
                 .conflicts_with("seed"))
        .arg(Arg::with_name("generator")
                 .short("g")
                 .long("generator")
                 .help("Generator function")
                 .possible_values(&["diamond", "fractal", "midpoint"])
                 .takes_value(true))
        .arg(Arg::with_name("noise")
                 .short("n")
                 .long("noise")
                 .help("noise type")
                 .takes_value(true)
                 .possible_values(&["value", "gradient", "simplex"]))
        .arg(Arg::with_name("interpolation")
                 .short("i")
                 .long("interpolation")
                 .help("Interpolation method")
                 .possible_values(&["linear", "cubic", "quintic", "cosine"])
                 .takes_value(true))
        .arg(Arg::with_name("output")
                 .short("o")
                 .long("output")
                 .help("generated file name")
                 .value_name("FILE"))
        .arg(Arg::with_name("write_config")
                 .long("write-config")
                 .takes_value(false)
                 .hidden(true)
                 .help("Write config file. It will be generated in the same folder as the output"))
        .group(ArgGroup::with_name("manual_group")
                   .args(&["generator", "noise", "interpolation"])
                   .multiple(true)
                   .conflicts_with("config"))
        .get_matches();

    let output = matches.value_of("output");
    let seed = if matches.is_present("random-seed") {
        Some(StdRng::new().unwrap().gen())
    } else {
        matches.value_of("seed").map(|s| usize::from_str(s).ok().unwrap())
    };

    let config = if matches.is_present("config") {
        config::MapGeneratorConfig::read(matches.value_of("config").unwrap())
            .expect("Could not read config file")
            .set_output(output.map(|o| o.to_string()))
            .set_seed(seed)
    } else if matches.value_of("generator").map_or(true, |g| g == "fractal") {
        config::MapGeneratorConfigBuilder::default()
            .width(config::default_width())
            .height(config::default_height())
            .generator(config::Generator::Fractal)
            .noise(Some(matches.value_of("noise")
                            .map(|s| FromStr::from_str(s).unwrap())
                            .unwrap_or(config::Noise::Gradient)))
            .scale(Some(2.0))
            .octave(Some(10))
            .lacunarity(Some(2.0))
            .persistance(Some(0.5))
            .interpolation(Some(matches.value_of("interpolation")
                                    .map(|s| FromStr::from_str(s).unwrap())
                                    .unwrap_or(config::Interpolation::Cubic)))
            .light_position(config::default_light_position())
            .light(config::default_light())
            .dark(config::default_dark())
            .output(output.map_or("out.png".to_string(), |filename| filename.to_string()))
            .seed(seed.unwrap_or_else(|| config::default_seed()))
            .ramp(config::default_ramp())
            .build()
            .unwrap()
    } else {
        config::MapGeneratorConfigBuilder::default()
            .width(config::default_width())
            .height(config::default_height())
            .generator(matches.value_of("generator")
                           .map(|s| FromStr::from_str(s).unwrap())
                           .unwrap_or(config::Generator::Midpoint))
            .noise(None)
            .scale(None)
            .octave(None)
            .lacunarity(None)
            .persistance(None)
            .interpolation(None)
            .light_position(config::default_light_position())
            .light(config::default_light())
            .dark(config::default_dark())
            .output(output.map_or("out.png".to_string(), |filename| filename.to_string()))
            .seed(seed.unwrap_or_else(|| config::default_seed()))
            .ramp(config::default_ramp())
            .build()
            .unwrap()
    };

    if matches.is_present("write_config") {
        let config_name = config.output().clone() + ".yml";
        let _ = config.write(&config_name);
    }
    println!("Seed used: {}", config.seed());
    map_generator::MapGenerator::new(config).run();
}
