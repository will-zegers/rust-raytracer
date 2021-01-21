use crate::color::Color;
use crate::geometry::Point3;

pub trait Texture {
    fn value(&self, u: f64, b: f64, p: &Point3) -> &Color;
}

pub struct SolidColor {
    pub color: Color,
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _b: f64, _p: &Point3) -> &Color {
        &self.color
    }
}
