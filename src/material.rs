use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3;
use crate::vec3::Vec3;

pub trait Material {
    fn scatter<'a>(&self, ray_in: &Ray, rec: &'a HitRecord) -> Option<Scatter<'a>>;
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
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter<'a>(&self, _ray_in: &Ray, rec: &'a HitRecord) -> Option<Scatter<'a>> {
        let mut scatter_direction = &rec.normal + vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.clone();
        }

        Some(Scatter {
            ray: Ray::new(&rec.p, scatter_direction),
            attenuation: self.albedo.clone(),
        })
    }
}

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
        let scattered = Ray::new(&rec.p, reflected + self.fuzz*vec3::random_in_unit_sphere());
        if vec3::dot(&scattered.direction, &rec.normal) <= 0. {
            return None;
        }
        Some(Scatter {
            ray: scattered,
            attenuation: self.albedo.clone(),
        })
    }
}
