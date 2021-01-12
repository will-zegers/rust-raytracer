use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;

pub trait Material {
    fn scatter<'a>(&self, ray_in: &Ray, rec: &'a HitRecord) -> Option<Scatter<'a>>;
}

pub struct Scatter<'a> {
    pub ray: Ray<'a>,
    pub attenuation: Color,
}
