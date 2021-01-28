use std::rc::Rc;

use super::{
    AxisAlignment, HitRecord, Hittable, HittableList, Point3, Ray, Rect, RectCorner, AABB,
};

use crate::material::Material;

pub struct Block {
    bbox: AABB,
    sides: HittableList,
}

impl Block {
    pub fn new(block_min: Point3, block_max: Point3, material: Rc<dyn Material>) -> Block {
        let mut sides = HittableList::new();
        sides.add(Box::new(Rect::new(
            AxisAlignment::XY,
            RectCorner(block_max.x, block_max.y),
            RectCorner(block_min.x, block_min.y),
            block_min.z,
            material.clone(),
        )));
        sides.add(Box::new(Rect::new(
            AxisAlignment::XY,
            RectCorner(block_max.x, block_max.y),
            RectCorner(block_min.x, block_min.y),
            block_max.z,
            material.clone(),
        )));
        sides.add(Box::new(Rect::new(
            AxisAlignment::XZ,
            RectCorner(block_max.x, block_max.z),
            RectCorner(block_min.x, block_min.z),
            block_min.y,
            material.clone(),
        )));
        sides.add(Box::new(Rect::new(
            AxisAlignment::XZ,
            RectCorner(block_max.x, block_max.z),
            RectCorner(block_min.x, block_min.z),
            block_max.y,
            material.clone(),
        )));
        sides.add(Box::new(Rect::new(
            AxisAlignment::YZ,
            RectCorner(block_max.y, block_max.z),
            RectCorner(block_min.y, block_min.z),
            block_min.x,
            material.clone(),
        )));
        sides.add(Box::new(Rect::new(
            AxisAlignment::YZ,
            RectCorner(block_max.y, block_max.z),
            RectCorner(block_min.y, block_min.z),
            block_max.x,
            material.clone(),
        )));
        Block {
            bbox: AABB {
                minimum: block_min,
                maximum: block_max,
            },
            sides,
        }
    }
}

impl Hittable for Block {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(&ray, t_min, t_max)
    }

    fn bounding_box(&self) -> Option<&AABB> {
        Some(&self.bbox)
    }
}
