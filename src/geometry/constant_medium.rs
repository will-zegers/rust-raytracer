use std::f64::{INFINITY, NEG_INFINITY};
use std::rc::Rc;

use rand::Rng;

use crate::color::Color;
use crate::geometry::{Ray, Vec3};
use crate::hittable::{HitRecord, Hittable, AABB};
use crate::material::types::Isotropic;
use crate::material::Material;

pub struct ConstantMedium {
    boundary: Box<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Rc<dyn Material>,
}

impl ConstantMedium {
    pub fn from_color(boundary: Box<dyn Hittable>, density: f64, color: Color) -> ConstantMedium {
        ConstantMedium {
            boundary,
            neg_inv_density: -1. / density,
            phase_function: Rc::new(Isotropic::from_color(color)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rng = rand::thread_rng();

        let mut rec1 = match self.boundary.hit(ray, NEG_INFINITY, INFINITY) {
            Some(rec) => rec,
            None => return None,
        };
        let mut rec2 = match self.boundary.hit(ray, rec1.t + 0.0001, INFINITY) {
            Some(rec) => rec,
            None => return None,
        };

        if rec1.t < t_min {
            rec1.t = t_min;
        }
        if rec2.t > t_max {
            rec2.t = t_max;
        }
        if rec1.t >= rec2.t {
            return None;
        }

        if rec1.t < 0. {
            rec1.t = 0.;
        }

        let ray_length = ray.direction.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * f64::ln(rng.gen());

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = rec1.t + hit_distance / ray_length;
        let p = ray.at(t);

        let rec = HitRecord::new(
            &ray,
            t,
            p,
            Vec3::new(0., 0., 0.),
            self.phase_function.clone(),
            0.,
            0.,
        );
        Some(rec)
    }

    fn bounding_box(&self) -> Option<&AABB> {
        self.boundary.bounding_box()
    }
}
