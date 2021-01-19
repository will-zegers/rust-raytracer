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

#[cfg(test)]
mod test {
    use super::*;

    use std::rc::Rc;

    use crate::geometry::{Hittable, Point3, Ray, Sphere, Vec3};

    #[test]
    fn test_lambertian_scatter() {
        let origin = Point3::new(0.0, 0.0, 0.0);
        let material_rc = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
        let sphere = Sphere::new(Point3::new(0., 0., -1.), 0.5, material_rc);

        let t_min = 0.;
        let t_max = std::f64::INFINITY;

        let r = Ray::new(origin.clone(), Vec3::new(0.0, 0.0, -1.0));
        let rec = sphere.hit(&r, t_min, t_max).unwrap();

        let scatter = rec.material_rc.scatter(&r, &rec).unwrap();
        assert_eq!(scatter.ray.origin, Vec3::new(0., 0., -0.5));
        assert_eq!(scatter.attenuation, Color::new(0.5, 0.5, 0.5));

        // the actual scatter direction is hard to predict, but it should always be a unit vector
        // (i.e. |v| == 1) added to the normal of the hit point
        assert!(f64::abs((scatter.ray.direction - rec.normal).length() - 1.) < Vec3::TOL);
    }
}
