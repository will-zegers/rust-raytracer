use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3;
use crate::vec3::Point3;

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let origin_to_center = ray.origin - &self.center;
        let a = ray.direction.length_squared();
        let half_b = vec3::dot(&origin_to_center, &ray.direction);
        let c = origin_to_center.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return false;
        }
        let sqrt_discrm = f64::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrt_discrm) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_discrm) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (&rec.p - &self.center) / self.radius;
        rec.set_face_normal(&ray, outward_normal);

        true
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::ray::Ray;
    use crate::vec3::{Point3, Vec3};

    #[test]
    fn test_sphere_hit() {
        let origin = Point3::new(0.0, 0.0, 0.0);
        let sphere = Sphere::new(Point3::new(0., 0., -1.), 0.5);

        let mut rec = HitRecord::new();
        let t_min = 0.;
        let t_max = std::f64::INFINITY;

        let r = Ray::new(&origin, Vec3::new(0.0, 0.0, -1.0));
        assert!(sphere.hit(&r, t_min, t_max, &mut rec));

        let r = Ray::new(&origin, Vec3::new(1.0, 1.0, 1.0));
        assert!(!sphere.hit(&r, t_min, t_max, &mut rec));
    }
}
