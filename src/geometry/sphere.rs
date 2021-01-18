use std::rc::Rc;

use super::{HitRecord, Hittable, Point3, Ray, Vec3, AABB};
use crate::material::Material;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material_rc: Rc<dyn Material>,
    bbox: AABB,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material_rc: Rc<dyn Material>) -> Self {
        let bbox = AABB {
            minimum: &center - Vec3::new(radius, radius, radius),
            maximum: &center + Vec3::new(radius, radius, radius),
        };
        Self {
            center,
            radius,
            material_rc,
            bbox,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let origin_to_center = &ray.origin - &self.center;
        let a = ray.direction.length_squared();
        let half_b = Vec3::dot(&origin_to_center, &ray.direction);
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
        let rec = HitRecord::new(
            &ray,
            root,
            ray.at(root),
            outward_normal,
            self.material_rc.clone(),
        );
        Some(rec)
    }

    fn bounding_box(&self) -> Option<&AABB> {
        Some(&self.bbox)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::color::Color;
    use crate::geometry::{Point3, Ray, Vec3};
    use crate::material::types::Lambertian;

    #[test]
    fn test_sphere_hit() {
        let mut rec: Option<HitRecord>;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let material_rc = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
        let sphere = Sphere::new(Point3::new(0., 0., -1.), 0.5, material_rc);

        let t_min = 0.;
        let t_max = std::f64::INFINITY;

        let r = Ray::new(origin.clone(), Vec3::new(0.0, 0.0, -1.0));
        rec = sphere.hit(&r, t_min, t_max);
        let hit = rec.is_some();
        assert!(hit);

        let r = Ray::new(origin.clone(), Vec3::new(1.0, 1.0, 1.0));
        rec = sphere.hit(&r, t_min, t_max);
        let miss = rec.is_none();
        assert!(miss);
    }
}
