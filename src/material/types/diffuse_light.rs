use crate::color::Color;
use crate::geometry::{HitRecord, Point3, Ray};
use crate::material::{Material, Scatter};
use crate::texture::{SolidColor, Texture};

pub struct DiffuseLight {
    pub emit: Box<SolidColor>,
}

impl DiffuseLight {
    pub fn new(color: Color) -> DiffuseLight {
        DiffuseLight {
            emit: Box::new(SolidColor { color }),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord) -> Option<Scatter> {
        None
    }

    fn emit(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.emit.value(u, v, p)
    }
}
