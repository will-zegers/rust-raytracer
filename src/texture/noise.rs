use super::{NoiseStrategy, Perlin, Texture};

use crate::color::Color;
use crate::geometry::Point3;

pub struct Noise {
    noise: Perlin,
    scale: f64,
}

impl Noise {
    pub fn new(noise_strategy: NoiseStrategy, scale: f64) -> Noise {
        Noise {
            noise: Perlin::new(noise_strategy),
            scale,
        }
    }
}

impl Texture for Noise {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        Color::new(1., 1., 1.)
            * 0.5
            * (1. + f64::sin(self.scale * p.z + 10. * self.noise.turbulence(&(self.scale * p), 7)))
    }
}
