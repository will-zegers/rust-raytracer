use std::f64::{INFINITY, NEG_INFINITY};
use std::f64::consts::PI;

use crate::geometry::{
    AABB,
    Hittable,
    HitRecord,
    Point3,
    Ray,
    Vec3,
};

pub struct Rotate {
    ptr: Box<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB,
}

impl Rotate {
    pub fn new(ptr: Box<dyn Hittable>, angle: f64) -> Rotate {
        let angle = angle * PI / 180.;
        let sin_theta = f64::sin(angle);
        let cos_theta = f64::cos(angle);
        let ptr_bbox = ptr.bounding_box().unwrap();

        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let i = i as f64;
                    let j = j as f64;
                    let k = k as f64;

                    let x = i*ptr_bbox.maximum.x + (1.-i)*ptr_bbox.minimum.x;
                    let y = j*ptr_bbox.maximum.y + (1.-j)*ptr_bbox.minimum.y;
                    let z = k*ptr_bbox.maximum.z + (1.-k)*ptr_bbox.minimum.z;

                    let x_rot = cos_theta*x + sin_theta*z;
                    let z_rot = -sin_theta*x + cos_theta*z;

                    let tester = Vec3::new(x_rot, y, z_rot);
                    for c in 0..3 {
                        min[c] = f64::min(min[c], tester[c]);
                        max[c] = f64::max(max[c], tester[c]);
                    }
                }
            }
        }
        let bbox = AABB {minimum: min, maximum: max};
        Rotate {
            ptr,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hittable for Rotate {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let rot_origin = Point3 {
            x: &self.cos_theta*ray.origin.x - &self.sin_theta*ray.origin.z,
            y: ray.origin.y,
            z: &self.sin_theta*ray.origin.x + &self.cos_theta*ray.origin.z,
        };
        let rot_direction = Vec3 {
            x: &self.cos_theta*ray.direction.x - &self.sin_theta*ray.direction.z,
            y: ray.direction.y,
            z: &self.sin_theta*ray.direction.x + &self.cos_theta*ray.direction.z,
        };
        let rot_ray = Ray::new(rot_origin, rot_direction);

        return match self.ptr.hit(&rot_ray, t_min, t_max) {
            Some(rec) => {
                let p = Point3 {
                    x: &self.cos_theta*rec.p.x + &self.sin_theta*rec.p.z,
                    y: rec.p.y,
                    z: -&self.sin_theta*rec.p.x + &self.cos_theta*rec.p.z,
                };
                let normal = Vec3 {
                    x: &self.cos_theta*rec.normal.x + &self.sin_theta*rec.normal.z,
                    y: rec.normal.y,
                    z: -&self.sin_theta*rec.normal.x + &self.cos_theta*rec.normal.z,
                };
                Some(HitRecord::new(
                    &rot_ray,
                    rec.t,
                    p,
                    normal,
                    rec.material_rc.clone(),
                    rec.u,
                    rec.v,
                ))
            },
            None => None
        }

    }

    fn bounding_box(&self) -> Option<&AABB> {
        Some(&self.bbox)
    }
}
