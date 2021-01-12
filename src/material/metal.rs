use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material;
use crate::material::base::{Material, Scatter};
use crate::ray::Ray;
use crate::vec3;

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        debug_assert!(0. <= fuzz && fuzz <= 1.);
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: & HitRecord) -> Option<Scatter> {
        let reflected = material::reflect(&ray_in.direction.unit_vector(), &rec.normal);
        let scattered = Ray::new(
            rec.p.clone(),
            reflected + self.fuzz * vec3::random_in_unit_sphere(),
        );
        if vec3::dot(&scattered.direction, &rec.normal) <= 0. {
            return None;
        }
        Some(Scatter {
            ray: scattered,
            attenuation: self.albedo.clone(),
        })
    }
}
