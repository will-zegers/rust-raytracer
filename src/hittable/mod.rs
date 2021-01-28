use crate::geometry::Ray;

pub mod aabb;
pub use aabb::AABB;

pub mod instance;

mod hittable_list;
pub use hittable_list::HittableList;

mod hit_record;
pub use hit_record::HitRecord;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self) -> Option<&AABB>;
}
