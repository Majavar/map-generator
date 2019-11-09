extern crate heightmap;
extern crate image;
extern crate noise2d;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod color;
mod color_ramp;
mod shade;
mod to_image;

pub use to_image::ToImage;
pub use color::Color;
pub use color_ramp::ColorRamp;
pub use shade::Shadable;
pub use shade::Vec3;
