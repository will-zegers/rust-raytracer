use std::rc::Rc;

use crate::material::Material;
use crate::geometry::{Point3, Ray, Vec3};

#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Point3,
    pub front_face: bool,
    pub normal: Vec3,
    pub material_rc: Rc<dyn Material>,
    pub u: f64,
    pub v: f64,
}

impl HitRecord {
    pub fn new(
        ray: &Ray,
        t: f64,
        p: Point3,
        normal: Vec3,
        material_rc: Rc<dyn Material>,
        u: f64,
        v: f64,
    ) -> HitRecord {
        let front_face = HitRecord::get_front_face(&ray, &normal);
        let normal = if front_face { normal } else { -normal };
        HitRecord {
            t,
            p,
            front_face,
            normal,
            material_rc,
            u,
            v,
        }
    }

    fn get_front_face(ray: &Ray, outward_normal: &Vec3) -> bool {
        Vec3::dot(&ray.direction, &outward_normal) < 0.
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use super::*;

    use crate::color::Color;
    use crate::texture::SolidColor;
    use crate::material::types::Lambertian;

    #[test]
    fn test_hitrecord_new() {
        let mut rec: HitRecord;

        let ray_origin = Vec3::new(0., 0., 0.);
        let ray_opposite_direction = Ray::new(ray_origin.clone(), Vec3::new(0.0, 0.0, -1.0));

        let t = 0.5;
        let p = Vec3::new(0.0, 0.0, -0.5);
        let normal = Vec3::new(0.0, 0.0, 1.0);
        let color = Rc::new(SolidColor {
            color: Color::new(0., 0., 0.),
        });
        let material_rc = Rc::new(Lambertian::new(color));

        rec = HitRecord::new(
            &ray_opposite_direction,
            t,
            p.clone(),
            normal.clone(),
            material_rc.clone(),
            0.,
            0.,
        );
        assert_eq!(rec.t, t);
        assert_eq!(&rec.p, &p);
        assert_eq!(rec.normal, normal);

        let ray_same_direction = Ray::new(ray_origin.clone(), Vec3::new(0.0, 0.0, 1.0));

        rec = HitRecord::new(
            &ray_same_direction,
            t,
            p.clone(),
            normal.clone(),
            material_rc.clone(),
            0.,
            0.,
        );
        assert_eq!(rec.t, t);
        assert_eq!(&rec.p, &p);
        assert_eq!(-rec.normal, normal);
    }

    #[test]
    fn test_hitrecord_get_front_face() {
        let origin = Point3::new(0., 0., 0.);
        let ray = Ray::new(origin, Vec3::new(0., 0., 0.5));

        let outward_normal_same = Vec3::new(0., 0., -0.5);
        let same_direction = HitRecord::get_front_face(&ray, &outward_normal_same);
        assert!(same_direction);

        let outward_normal_opposite = Vec3::new(0., 0., 0.5);
        let opposite_direction = !HitRecord::get_front_face(&ray, &outward_normal_opposite);
        assert!(opposite_direction);
    }
}
