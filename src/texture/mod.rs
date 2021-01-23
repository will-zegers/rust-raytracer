use crate::color::Color;
use crate::geometry::Point3;

mod checker;
pub use checker::Checker;

mod noise;
pub use noise::Noise;

mod perlin;
pub use perlin::NoiseStrategy;
use perlin::Perlin;

mod solid_color;
pub use solid_color::SolidColor;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}
