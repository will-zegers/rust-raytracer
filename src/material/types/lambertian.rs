use crate::color::Color;
use crate::geometry::{HitRecord, RandomVectorType, Ray, Vec3};
use crate::material::{Material, Scatter};

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let mut scatter_direction = &rec.normal + Vec3::random(RandomVectorType::Unit);

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.clone();
        }

        Some(Scatter {
            ray: Ray::new(rec.p.clone(), scatter_direction),
            attenuation: self.albedo.clone(),
        })
    }
}
