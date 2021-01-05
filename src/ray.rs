
use crate::color::Color;
use crate::vec3;
use crate::vec3::{Point3, Vec3};

#[allow(dead_code)]
pub struct Ray<'a> {
    origin: &'a Point3,
    direction: Vec3,
}

impl<'a> Ray<'a> {
    pub fn new(origin: &'a Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    #[allow(dead_code)]
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * &self.direction
    }

    pub fn color(&self) -> Color {
        let sphere_center = Point3::new(0.0, 0.0, -1.0);
        let radius = 0.5;
        if hit_sphere(sphere_center, radius, &self) {
            return Color::new(1.0, 0.0, 0.0)
        }
        let unit_direction = self.direction.unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

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

    let r = Ray::new(&origin, Vec3::new(0.0, -1.0, 0.0));
    let c_white = r.color();
    assert_eq!(c_white, Color::new(1.0, 1.0, 1.0));

    let r = Ray::new(&origin, Vec3::new(1.0, 0.0, 1.0));
    let c_mid = r.color();
    assert_eq!(c_mid, Color::new(0.75, 0.85, 1.0));

    let r = Ray::new(&origin, Vec3::new(0.0, 1.0, 0.0));
    let c_blue = r.color();
    assert_eq!(c_blue, Color::new(0.5, 0.7, 1.0));
}

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> bool {
    let origin_to_center = ray.origin - center;
    let a = vec3::dot(&ray.direction, &ray.direction);
    let b = 2.0 * vec3::dot(&origin_to_center, &ray.direction);
    let c = vec3::dot(&origin_to_center, &origin_to_center) - radius*radius;

    let discriminant = b*b - 4.0*a*c;
    discriminant > 0.0
}

#[test]
fn test_ray_hit_sphere() {
    let origin = Point3::new(0.0, 0.0, 0.0);

    let r = Ray::new(&origin, Vec3::new(0.0, 0.0, -1.0));
    let c_hit = r.color();
    assert_eq!(c_hit, Color::new(1.0, 0.0, 0.0));

    let r = Ray::new(&origin, Vec3::new(1.0, 1.0, 0.0));
    let c_miss = r.color();
    assert_ne!(c_miss, Color::new(1.0, 0.0, 0.0));
}
