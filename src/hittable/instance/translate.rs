use super::super::*;

use crate::geometry::{
    Ray,
    Vec3,
};
use crate::hittable::{AABB, Hittable};

pub struct Translate {
    ptr: Box<dyn Hittable>,
    offset: Vec3,
    bbox: AABB,
}

impl Translate {
    pub fn new(ptr: Box<dyn Hittable>, offset: Vec3) -> Translate {
        let ptr_bbox = ptr.bounding_box().unwrap();
        let bbox = AABB {
            minimum: &ptr_bbox.minimum + &offset,
            maximum: &ptr_bbox.maximum + &offset,
        };
        Translate {
            ptr,
            offset,
            bbox,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let ray_moved = Ray::new(&ray.origin - &self.offset, ray.direction.clone());
        return match self.ptr.hit(&ray_moved, t_min, t_max) {
            Some(rec) => {
                Some(HitRecord::new(
                    &ray_moved,
                    rec.t,
                    rec.p + &self.offset,
                    rec.normal,
                    rec.material_rc,
                    rec.u,
                    rec.v,
                ))
            },
            None => None,
        }
    }

    fn bounding_box(&self) -> Option<&AABB> {
        return match self.ptr.bounding_box() {
            Some(_) => Some(&self.bbox),
            None => None,
        }
    }
}
