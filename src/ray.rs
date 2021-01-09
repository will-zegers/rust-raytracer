use crate::color;
use crate::color::Color;

use crate::hittable::{HitRecord, Hittable};

use crate::vec3::{Point3, Vec3};

pub struct Ray<'a> {
    pub origin: &'a Point3,
    pub direction: Vec3,
}

impl<'a> Ray<'a> {
    pub fn new(origin: &'a Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * &self.direction
    }

    pub fn color(&self, world: &dyn Hittable) -> Color {
        let mut rec = HitRecord::new();
        if world.hit(&self, 0., std::f64::INFINITY, &mut rec) {
            return 0.5 * (color::vec3_to_color(rec.normal) + Color::new(1., 1., 1.));
        }
        let unit_direction = self.direction.unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::sphere::Sphere;

    #[test]
    fn test_ray_new() {
        let p = Point3::new(1.1, 2.2, 3.3);
        let v = Vec3::new(4.4, 5.5, 6.6);
        let r = Ray::new(&p, v);

        assert_eq!(r.origin.x(), 1.1);
        assert_eq!(r.origin.y(), 2.2);
        assert_eq!(r.origin.z(), 3.3);
        assert_eq!(r.direction.x(), 4.4);
        assert_eq!(r.direction.y(), 5.5);
        assert_eq!(r.direction.z(), 6.6);
    }

    #[test]
    fn test_ray_at() {
        let p = Point3::new(0.0, 0.0, 0.0);
        let v = Vec3::new(2.0, 4.0, 8.0);
        let r = Ray::new(&p, v);

        let point3_at = r.at(3.0);
        assert_eq!(point3_at, Point3::new(6.0, 12.0, 24.0));
    }

    #[test]
    fn test_ray_color() {
        let origin = Point3::new(0.0, 0.0, 0.0);
        let sphere = Sphere::new(Point3::new(0., 0., -1.), 0.5);

        let r = Ray::new(&origin, Vec3::new(0.0, -1.0, 0.0));
        let c_white = r.color(&sphere);
        assert_eq!(c_white, Color::new(1.0, 1.0, 1.0));

        let r = Ray::new(&origin, Vec3::new(1.0, 0.0, 1.0));
        let c_mid = r.color(&sphere);
        assert_eq!(c_mid, Color::new(0.75, 0.85, 1.0));

        let r = Ray::new(&origin, Vec3::new(0.0, 1.0, 0.0));
        let c_blue = r.color(&sphere);
        assert_eq!(c_blue, Color::new(0.5, 0.7, 1.0));
    }

    #[test]
    fn test_ray_hit_sphere() {
        let origin = Point3::new(0.0, 0.0, 0.0);
        let sphere = Sphere::new(Point3::new(0., 0., -1.), 0.5);

        let r = Ray::new(&origin, Vec3::new(0.0, 0.0, -1.0));
        let c_hit = r.color(&sphere);
        assert_eq!(c_hit, Color::new(0.5, 0.5, 1.0));

        let r = Ray::new(&origin, Vec3::new(1.0, 1.0, 0.0));
        let c_miss = r.color(&sphere);
        assert_ne!(c_miss, Color::new(1.0, 0.0, 0.0));
    }
}
