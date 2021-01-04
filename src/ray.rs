
use crate::color::Color;
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
    assert_eq!(point3_at.x(), 6.0);
    assert_eq!(point3_at.y(), 12.0);
    assert_eq!(point3_at.z(), 24.0);
}

#[test]
fn test_ray_color() {
    let origin = Point3::new(0.0, 0.0, 0.0);
    let r = Ray::new(&origin, Vec3::new(0.0, -1.0, 0.0));
    let c_white = r.color();
    assert_eq!(c_white.r(), 1.0);
    assert_eq!(c_white.g(), 1.0);
    assert_eq!(c_white.b(), 1.0);

    let r = Ray::new(&origin, Vec3::new(1.0, 0.0, 1.0));
    let c_mid = r.color();
    assert_eq!(c_mid.r(), 0.75);
    assert_eq!(c_mid.g(), 0.85);
    assert_eq!(c_mid.b(), 1.0);

    let r = Ray::new(&origin, Vec3::new(0.0, 1.0, 0.0));
    let c_blue = r.color();
    assert_eq!(c_blue.r(), 0.5);
    assert_eq!(c_blue.g(), 0.7);
    assert_eq!(c_blue.b(), 1.0);
}
