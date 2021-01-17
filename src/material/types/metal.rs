use crate::color::Color;
use crate::geometry::{HitRecord, RandomVectorType, Ray, Vec3};
use crate::material;
use crate::material::{Material, Scatter};

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
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let reflected = material::reflect(&ray_in.direction.unit_vector(), &rec.normal);
        let scattered = Ray::new(
            rec.p.clone(),
            reflected + self.fuzz * Vec3::random(RandomVectorType::InUnitSphere),
        );
        if Vec3::dot(&scattered.direction, &rec.normal) <= 0. {
            return None;
        }
        Some(Scatter {
            ray: scattered,
            attenuation: self.albedo.clone(),
        })
    }
}
