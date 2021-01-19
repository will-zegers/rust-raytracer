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

#[cfg(test)]
mod test {
    use super::*;

    use std::rc::Rc;

    use crate::geometry::{Hittable, Point3, Ray, Sphere, Vec3};

    #[test]
    fn test_metal_scatter() {
        let origin = Point3::new(0.0, 0.0, 0.0);
        let fuzz = 0.5;
        let material_rc = Rc::new(Metal::new(Color::new(0.5, 0.5, 0.5), fuzz));
        let sphere = Sphere::new(Point3::new(0., 0., -1.), 0.5, material_rc);

        let t_min = 0.;
        let t_max = std::f64::INFINITY;

        let r = Ray::new(origin.clone(), Vec3::new(0.0, 0.0, -1.0));
        let rec = sphere.hit(&r, t_min, t_max).unwrap();

        let scatter = rec.material_rc.scatter(&r, &rec).unwrap();
        assert_eq!(scatter.ray.origin, Vec3::new(0., 0., -0.5));
        assert_eq!(scatter.attenuation, Color::new(0.5, 0.5, 0.5));

        // the scattered ray direction is a bit tough to assert. this just makes sure the random
        // scattering is within the unit sphere (i.e. |v| < 1)
        let reflected = material::reflect(&r.direction.unit_vector(), &rec.normal);
        assert!(((scatter.ray.direction - reflected) / fuzz).length() < 1.);
    }
}
