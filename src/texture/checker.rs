use super::Texture;

use crate::color::Color;
use crate::geometry::Point3;

pub struct Checker {
    pub odd: Box<dyn Texture>,
    pub even: Box<dyn Texture>,
}

impl Texture for Checker {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let sines = f64::sin(10. * p.x) * f64::sin(10. * p.y) * f64::sin(10. * p.z);

        return if sines < 0. {
            self.odd.value(u, v, &p)
        } else {
            self.even.value(u, v, &p)
        };
    }
}
