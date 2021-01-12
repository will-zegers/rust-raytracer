use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Color,
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<Scatter>;
}
