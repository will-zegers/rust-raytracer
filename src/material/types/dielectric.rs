use std::rc::Rc;

use rand::Rng;

use crate::color::Color;
use crate::geometry::{Ray, Vec3};
use crate::hittable::HitRecord;
use crate::material;
use crate::material::{Material, Scatter};
use crate::texture::{SolidColor, Texture};

pub struct Dielectric {
    albedo: Rc<dyn Texture>,
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            albedo: Rc::new(SolidColor {
                color: Color::new(1., 1., 1.),
            }),
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
        let cos_theta = f64::min(Vec3::dot(&-&unit_direction, &rec.normal), 1.0);
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
            attenuation: self.albedo.clone(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::rc::Rc;

    use crate::geometry::{Point3, Ray, Sphere, Vec3};
    use crate::hittable::Hittable;

    #[test]
    fn test_dielectric_scatter() {
        let origin = Point3::new(0.0, 0.0, 0.0);
        let material_rc = Rc::new(Dielectric::new(1.5));
        let sphere = Sphere::new(Point3::new(0., 0., -1.), 0.5, material_rc);

        let t_min = 0.;
        let t_max = std::f64::INFINITY;

        let r = Ray::new(origin.clone(), Vec3::new(0.0, 0.0, -1.0));
        let rec = sphere.hit(&r, t_min, t_max).unwrap();

        let scatter = rec.material_rc.scatter(&r, &rec).unwrap();
        assert_eq!(scatter.ray.origin, Vec3::new(0., 0., -0.5));
        assert_eq!(
            scatter.attenuation.value(rec.u, rec.v, &rec.p),
            Color::new(1., 1., 1.)
        );

        // the material could either reflect or refract, so accept either for testing
        let direction = Vec3::new(0., 0., -1.);
        assert!(scatter.ray.direction == direction || scatter.ray.direction == -direction);
    }
}
