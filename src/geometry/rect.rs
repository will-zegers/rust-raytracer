use std::rc::Rc;

use crate::geometry::{HitRecord, Hittable, Point3, Ray, Vec3, AABB};
use crate::material::Material;

#[derive(Clone, Copy)]
pub enum AxisAlignment {
    XY,
    XZ,
    YZ,
}

pub struct RectCorner(pub f64, pub f64);

pub struct Rect {
    axes: AxisAlignment,
    upper_left: RectCorner,
    lower_right: RectCorner,
    normal_plane: f64,
    material: Rc<dyn Material>,
    bbox: AABB,
}

impl Rect {
    pub fn new(
        axes: AxisAlignment,
        upper_left: RectCorner,
        lower_right: RectCorner,
        normal_plane: f64,
        material: Rc<dyn Material>,
    ) -> Rect {
        let bbox = get_bounding_box(axes, &lower_right, &upper_left, normal_plane);
        Rect {
            axes,
            upper_left,
            lower_right,
            normal_plane,
            material,
            bbox,
        }
    }
}

impl Hittable for Rect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let (p1, p2, t): (f64, f64, f64);
        let outward_normal: Vec3;

        match self.axes {
            AxisAlignment::XY => {
                t = (self.normal_plane - ray.origin.z) / ray.direction.z;
                outward_normal = Vec3::new(0., 0., 1.);
                p1 = ray.origin.x + t * ray.direction.x;
                p2 = ray.origin.y + t * ray.direction.y;
            }
            AxisAlignment::XZ => {
                t = (self.normal_plane - ray.origin.y) / ray.direction.y;
                outward_normal = Vec3::new(0., 1., 0.);
                p1 = ray.origin.x + t * ray.direction.x;
                p2 = ray.origin.z + t * ray.direction.z;
            }
            AxisAlignment::YZ => {
                t = (self.normal_plane - ray.origin.x) / ray.direction.x;
                outward_normal = Vec3::new(1., 0., 0.);
                p1 = ray.origin.y + t * ray.direction.y;
                p2 = ray.origin.z + t * ray.direction.z;
            }
        }

        if t < t_min || t > t_max {
            return None;
        }
        if p1 < self.lower_right.0 || p1 > self.upper_left.0 {
            return None;
        }
        if p2 < self.lower_right.1 || p2 > self.upper_left.1 {
            return None;
        }

        let rec = HitRecord::new(
            &ray,
            t,
            ray.at(t),
            outward_normal,
            self.material.clone(),
            (p1 - self.lower_right.0) / (self.upper_left.0 - self.lower_right.0),
            (p2 - self.lower_right.1) / (self.upper_left.1 - self.lower_right.1),
        );

        Some(rec)
    }

    fn bounding_box(&self) -> Option<&AABB> {
        Some(&self.bbox)
    }
}

fn get_bounding_box(
    axes: AxisAlignment,
    lower_right: &RectCorner,
    upper_left: &RectCorner,
    normal_plane: f64,
) -> AABB {
    let (minimum, maximum) = match axes {
        AxisAlignment::XY => (
            Point3::new(lower_right.0, lower_right.1, normal_plane - 0.0001),
            Point3::new(upper_left.0, upper_left.1, normal_plane + 0.0001),
        ),
        AxisAlignment::XZ => (
            Point3::new(lower_right.0, normal_plane - 0.0001, lower_right.1),
            Point3::new(upper_left.0, normal_plane + 0.0001, upper_left.1),
        ),
        AxisAlignment::YZ => (
            Point3::new(normal_plane - 0.0001, lower_right.0, lower_right.1),
            Point3::new(normal_plane + 0.0001, upper_left.0, upper_left.1),
        ),
    };
    AABB { minimum, maximum }
}
