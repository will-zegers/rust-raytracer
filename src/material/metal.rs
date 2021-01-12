use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::base::{Material, Scatter};
use crate::ray::Ray;
use crate::vec3;
use crate::vec3::Vec3;

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl<'a> Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        debug_assert!(0. <= fuzz && fuzz <= 1.);
        Self { albedo, fuzz }
    }

    fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        v - 2.0 * vec3::dot(v, n) * n
    }
}

impl Material for Metal {
    fn scatter<'a>(&self, ray_in: &Ray, rec: &'a HitRecord) -> Option<Scatter<'a>> {
        let reflected = Metal::reflect(&ray_in.direction.unit_vector(), &rec.normal);
        let scattered = Ray::new(
            &rec.p,
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
