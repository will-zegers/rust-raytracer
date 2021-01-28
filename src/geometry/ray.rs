use super::{Point3, Vec3};

use crate::color::Color;
use crate::hittable::Hittable;

#[derive(Debug, PartialEq)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        &self.origin + t * &self.direction
    }

    pub fn color(&self, world: &dyn Hittable, depth: i32, background: &Color) -> Color {
        // If we've exceeded the ray bounce lmit, no more light is gathered.
        if depth <= 0 {
            return Color::new(0., 0., 0.);
        }

        match world.hit(&self, 0.001, std::f64::INFINITY) {
            Some(rec) => match rec.material_rc.scatter(&self, &rec) {
                Some(scatter) => {
                    let emitted = rec.material_rc.emit(rec.u, rec.v, &rec.p);
                    return emitted
                        + scatter.attenuation.value(rec.u, rec.v, &rec.p)
                            * scatter.ray.color(world, depth - 1, background);
                }
                None => {
                    return rec.material_rc.emit(rec.u, rec.v, &rec.p);
                }
            },
            // If the ray hits nothing, return the background color
            None => return background.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use super::*;

    use crate::hittable::HitRecord;
    use crate::geometry::Sphere;
    use crate::material::types::Lambertian;
    use crate::texture::SolidColor;

    #[test]
    fn test_ray_new() {
        let p = Point3::new(1.1, 2.2, 3.3);
        let v = Vec3::new(4.4, 5.5, 6.6);
        let r = Ray::new(p, v);

        assert_eq!(r.origin, Vec3::new(1.1, 2.2, 3.3));
        assert_eq!(r.direction, Vec3::new(4.4, 5.5, 6.6));
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
        let color = Box::new(SolidColor {
            color: Color::new(0.5, 0.5, 0.5),
        });
        let material_rc = Rc::new(Lambertian::new(color));
        let sphere = Sphere::new(Point3::new(0., 0., -1.), 0.5, material_rc);

        let depth = 10;
        let background = Color::new(0.5, 0.7, 1.0);

        let r = Ray::new(origin.clone(), Vec3::new(0.0, -1.0, 0.0));
        assert!(sphere.hit(&r, 0.001, std::f64::INFINITY).is_none());
        let c_bg = r.color(&sphere, depth, &background);
        assert_eq!(c_bg, background);

        let r = Ray::new(origin.clone(), Vec3::new(0.0, 0.0, -1.0));
        assert!(sphere.hit(&r, 0.001, std::f64::INFINITY).is_some());
        let c_fg = r.color(&sphere, depth, &background);
        assert_eq!(c_fg, Color::new(0.25, 0.35, 0.5));
    }

    #[test]
    fn test_ray_hit_sphere() {
        let mut rec: Option<HitRecord>;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let color = Box::new(SolidColor {
            color: Color::new(0.5, 0.5, 0.5),
        });
        let material_rc = Rc::new(Lambertian::new(color));
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
