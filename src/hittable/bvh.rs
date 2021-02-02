// TODO: unit tests

#![allow(dead_code)]

use std::cmp::Ordering;
use std::rc::Rc;

use rand::Rng;

use super::Ray;
use crate::hittable::{HitRecord, Hittable, AABB};

pub struct BVHNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: AABB,
}

impl BVHNode {
    pub fn new(src_objects: &Vec<Rc<dyn Hittable>>, start: usize, end: usize) -> BVHNode {
        let left;
        let right;
        let mut objects = src_objects.clone();

        let comparator = match rand::thread_rng().gen_range(0..3) {
            0 => |a: &_, b: &_| BVHNode::box_compare(a, b, 0),
            1 => |a: &_, b: &_| BVHNode::box_compare(a, b, 1),
            _ => |a: &_, b: &_| BVHNode::box_compare(a, b, 2),
        };

        let object_span = end - start;
        match object_span {
            1 => {
                left = objects[start].clone();
                right = objects[start].clone();
            }
            2 => {
                if comparator(&objects[start], &objects[start + 1]) == Ordering::Less {
                    left = objects[start].clone();
                    right = objects[start + 1].clone();
                } else {
                    left = objects[start + 1].clone();
                    right = objects[start].clone();
                }
            }
            _ => {
                let mid = start + object_span / 2;

                objects.sort_by(comparator);
                left = Rc::new(BVHNode::new(&objects, start, mid));
                right = Rc::new(BVHNode::new(&objects, mid, end));
            }
        }

        let lbbox = left
            .bounding_box()
            .expect("No bounding box in BVHNode constructor");
        let rbbox = right
            .bounding_box()
            .expect("No bounding box in BVHNode constructor");
        return BVHNode {
            left: left.clone(),
            right: right.clone(),
            bbox: AABB::surrounding_box(lbbox, rbbox),
        };
    }

    fn box_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis: usize) -> Ordering {
        let box_a = a
            .bounding_box()
            .expect("No bounding box in BVHNode constructor");
        let box_b = b
            .bounding_box()
            .expect("No bounding box in BVHNode constructor");

        box_a.minimum[axis]
            .partial_cmp(&box_b.minimum[axis])
            .unwrap()
    }
}

impl Hittable for BVHNode {
    fn bounding_box(&self) -> Option<&AABB> {
        Some(&self.bbox)
    }

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bbox.hit(ray, t_min, t_max) {
            return None;
        }

        match self.left.hit(ray, t_min, t_max) {
            Some(lrec) => match self.right.hit(ray, t_min, lrec.t) {
                Some(rrec) => return Some(rrec),
                None => return Some(lrec),
            },
            None => return self.right.hit(ray, t_min, t_max),
        }
    }
}
