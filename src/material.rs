use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3;

pub trait Material {
    fn scatter <'a> (
        &self,
        ray_in: &Ray,
        rec: &'a HitRecord,
    ) -> Option<Scatter<'a>>;
}

pub struct Scatter<'a> {
    pub ray: Ray<'a>,
    pub attenuation: Color,
}

pub struct Lambertian {
    albedo: Color,
}

impl<'a> Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

 impl Material for Lambertian {
    fn scatter <'a>(&self, _ray_in: &Ray, rec: &'a HitRecord) -> Option<Scatter<'a>> {
        let mut scatter_direction = &rec.normal + vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.clone();
        }

        Some(Scatter {
            ray: Ray::new(&rec.p, scatter_direction),
            attenuation: self.albedo.clone(),
        })
    }
}
