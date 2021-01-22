use super::{Perlin, Texture};

use crate::color::Color;
use crate::geometry::Point3;

pub struct Noise {
    noise: Perlin,
}

impl Noise {
    pub fn new() -> Noise {
        Noise {
            noise: Perlin::new(),
        }
    }
}

impl Texture for Noise {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        Color::new(1., 1., 1.) * self.noise.noise(p)
    }
}
