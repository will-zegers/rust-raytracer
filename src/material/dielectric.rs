use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::base::{Material, Scatter};
use crate::ray::Ray;
use crate::vec3;
use crate::vec3::Vec3;

pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Dielectric {
        Dielectric {
            index_of_refraction,
        }
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = f64::min(vec3::dot(&(-uv), &n), 1.0);
        let r_out_perp = etai_over_etat * (uv + cos_theta * n);
        let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())) * n;

        r_out_perp + r_out_parallel
    }
}

impl Material for Dielectric {
    fn scatter<'a>(&self, ray_in: &Ray, rec: &'a HitRecord) -> Option<Scatter<'a>> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = ray_in.direction.unit_vector();
        let refracted = Dielectric::refract(&unit_direction, &rec.normal, refraction_ratio);

        Some(Scatter {
            ray: Ray::new(&rec.p, refracted),
            attenuation: Color::new(1., 1., 1.),
        })
    }
}
