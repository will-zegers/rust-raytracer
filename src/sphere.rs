use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3;
use crate::vec3::Point3;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Sphere {
        Sphere { center, radius, material }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let origin_to_center = ray.origin - &self.center;
        let a = ray.direction.length_squared();
        let half_b = vec3::dot(&origin_to_center, &ray.direction);
        let c = origin_to_center.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }
        let sqrt_discrm = f64::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrt_discrm) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_discrm) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let outward_normal = (&ray.at(root) - &self.center) / self.radius;
        Some(HitRecord {
            t: root,
            normal: HitRecord::get_face_normal(&ray, outward_normal),
            p: ray.at(root),
            material: self.material.clone(),
        })
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::color::Color;
    use crate::material::Lambertian;
    use crate::ray::Ray;
    use crate::vec3::{Point3, Vec3};

    #[test]
    fn test_sphere_hit() {
        let mut rec: Option<HitRecord>;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
        let sphere = Sphere::new(Point3::new(0., 0., -1.), 0.5, material);

        let t_min = 0.;
        let t_max = std::f64::INFINITY;

        let r = Ray::new(&origin, Vec3::new(0.0, 0.0, -1.0));
        rec = sphere.hit(&r, t_min, t_max);
        let hit = rec.is_some();
        assert!(hit);

        let r = Ray::new(&origin, Vec3::new(1.0, 1.0, 1.0));
        rec = sphere.hit(&r, t_min, t_max);
        let miss = rec.is_none();
        assert!(miss);
    }
}
