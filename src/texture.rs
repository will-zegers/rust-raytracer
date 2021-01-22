use crate::color::Color;
use crate::geometry::Point3;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> &Color;
}

pub struct SolidColor {
    pub color: Color,
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> &Color {
        &self.color
    }
}

pub struct CheckerTexture {
    pub odd: Box<dyn Texture>,
    pub even: Box<dyn Texture>,
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> &Color {
        let sines = f64::sin(10. * p.x) * f64::sin(10. * p.y) * f64::sin(10. * p.z);

        return if sines < 0. {
            self.odd.value(u, v, &p)
        } else {
            self.even.value(u, v, &p)
        };
    }
}
