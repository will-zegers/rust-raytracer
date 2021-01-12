#[allow(unused_imports)]
use rand::Rng;

use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material;
use crate::material::base::{Material, Scatter};
use crate::ray::Ray;
use crate::vec3;

pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Dielectric {
        Dielectric {
            index_of_refraction,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = ray_in.direction.unit_vector();
        let cos_theta = f64::min(vec3::dot(&-&unit_direction, &rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        // A little extra-overhead creating a separate ThreadRng on each call, but overall not too
        // expensive
        let direction = if cannot_refract
            || material::reflectance(cos_theta, refraction_ratio) > rand::thread_rng().gen::<f64>()
        {
            material::reflect(&unit_direction, &rec.normal)
        } else {
            material::refract(&unit_direction, &rec.normal, refraction_ratio)
        };

        Some(Scatter {
            ray: Ray::new(rec.p.clone(), direction),
            attenuation: Color::new(1., 1., 1.),
        })
    }
}
