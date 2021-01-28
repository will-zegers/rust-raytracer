use std::rc::Rc;

use crate::color::Color;
use crate::geometry::{RandomVectorType, Ray, Vec3};
use crate::hittable::HitRecord;
use crate::material::{Material, Scatter};
use crate::texture::{SolidColor, Texture};

pub struct Isotropic {
    albedo: Rc<dyn Texture>,
}

impl Isotropic {
    pub fn from_color(color: Color) -> Self {
        Self {
            albedo: Rc::new(SolidColor { color }),
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let color = self.albedo.value(rec.u, rec.v, &rec.p);
        Some(Scatter {
            ray: Ray::new(rec.p.clone(), Vec3::random(RandomVectorType::InUnitSphere)),
            attenuation: Rc::new(SolidColor { color }),
        })
    }
}
