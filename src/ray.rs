use crate::color::Color;

use crate::hittable::Hittable;

use crate::vec3::{Point3, Vec3};

#[derive(Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        &self.origin + t * &self.direction
    }

    pub fn color(&self, world: &dyn Hittable, depth: i32) -> Color {
        // If we've exceeded the ray bounce lmit, no more light is gathered.
        if depth <= 0 {
            return Color::new(0., 0., 0.);
        }

        match world.hit(&self, 0.001, std::f64::INFINITY) {
            Some(rec) => match rec.material_rc.scatter(&self, &rec) {
                Some(scatter) => {
                    return scatter.attenuation * scatter.ray.color(world, depth - 1);
                }
                None => {
                    return Color::new(0., 0., 0.);
                }
            },
            None => (),
        }

        let unit_direction = self.direction.unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

impl PartialEq for Ray {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.direction == other.direction
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::hittable::HitRecord;
    use crate::material::lambertian::Lambertian;
    use crate::sphere::Sphere;
    use std::rc::Rc;

    #[test]
    fn test_ray_new() {
        let p = Point3::new(1.1, 2.2, 3.3);
        let v = Vec3::new(4.4, 5.5, 6.6);
        let r = Ray::new(p, v);

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
        let r = Ray::new(p, v);

        let point3_at = r.at(3.0);
        assert_eq!(point3_at, Point3::new(6.0, 12.0, 24.0));
    }

    #[test]
    fn test_ray_color() {
        let origin = Point3::new(0.0, 0.0, 0.0);
        let material_rc = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
        let sphere = Sphere::new(Point3::new(0., 0., -1.), 0.5, material_rc);

        let depth = 1;

        let r = Ray::new(origin.clone(), Vec3::new(0.0, -1.0, 0.0));
        let c_white = r.color(&sphere, depth);
        assert_eq!(c_white, Color::new(1.0, 1.0, 1.0));

        let r = Ray::new(origin.clone(), Vec3::new(1.0, 0.0, 1.0));
        let c_mid = r.color(&sphere, depth);
        assert_eq!(c_mid, Color::new(0.75, 0.85, 1.0));

        let r = Ray::new(origin.clone(), Vec3::new(0.0, 1.0, 0.0));
        let c_blue = r.color(&sphere, depth);
        assert_eq!(c_blue, Color::new(0.5, 0.7, 1.0));
    }

    #[test]
    fn test_ray_hit_sphere() {
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
