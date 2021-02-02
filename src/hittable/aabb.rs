#![allow(dead_code)]
// TODO: add unit tests

use crate::geometry::{Point3, Ray};

#[derive(Clone, Debug, PartialEq)]
pub struct AABB {
    pub minimum: Point3,
    pub maximum: Point3,
}

impl AABB {
    pub fn hit(&self, r: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for i in 0..3 {
            let t0 = f64::min(
                (self.minimum[i] - r.origin[i]) / r.direction[i],
                (self.maximum[i] - r.origin[i]) / r.direction[i],
            );
            let t1 = f64::max(
                (self.minimum[i] - r.origin[i]) / r.direction[i],
                (self.maximum[i] - r.origin[i]) / r.direction[i],
            );

            t_min = f64::max(t0, t_min);
            t_max = f64::max(t1, t_max);

            if t_max <= t_min {
                return false;
            }
        }

        true
    }

    pub fn surrounding_box(box0: &Self, box1: &Self) -> Self {
        let minimum = Point3::new(
            f64::min(box0.minimum.x, box1.minimum.x),
            f64::min(box0.minimum.y, box1.minimum.y),
            f64::min(box0.minimum.z, box1.minimum.z),
        );
        let maximum = Point3::new(
            f64::max(box0.maximum.x, box1.maximum.x),
            f64::max(box0.maximum.y, box1.maximum.y),
            f64::max(box0.maximum.z, box1.maximum.z),
        );

        AABB { minimum, maximum }
    }
}
