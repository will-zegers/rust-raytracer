use super::Texture;

use crate::color::Color;
use crate::geometry::Point3;

pub struct SolidColor {
    pub color: Color,
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.color.clone()
    }
}
