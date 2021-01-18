use std::rc::Rc;

use super::{Point3, Ray, Vec3, AABB};

use crate::material::Material;

#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Point3,
    pub front_face: bool,
    pub normal: Vec3,
    pub material_rc: Rc<dyn Material>,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self) -> Option<&AABB>;
}

impl HitRecord {
    pub fn new(
        ray: &Ray,
        t: f64,
        p: Point3,
        normal: Vec3,
        material_rc: Rc<dyn Material>,
    ) -> HitRecord {
        let front_face = HitRecord::get_front_face(&ray, &normal);
        let normal = if front_face { normal } else { -normal };
        HitRecord {
            t,
            p,
            front_face,
            normal,
            material_rc,
        }
    }

    fn get_front_face(ray: &Ray, outward_normal: &Vec3) -> bool {
        Vec3::dot(&ray.direction, &outward_normal) < 0.
    }
}

pub struct HittableList {
    bbox: AABB,
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            bbox: AABB {
                minimum: Point3::new(0., 0., 0.),
                maximum: Point3::new(0., 0., 0.),
            },
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        if object.bounding_box().is_some() {
            let obj_bbox = object.bounding_box().unwrap();
            if self.objects.len() == 0 {
                self.bbox = obj_bbox.clone();
            } else {
                self.bbox = AABB::surrounding_box(&self.bbox, obj_bbox)
            }
        }
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut rec: Option<HitRecord> = None;

        for object in &self.objects {
            match object.hit(ray, t_min, closest_so_far) {
                Some(temp_rec) => {
                    closest_so_far = temp_rec.t;
                    rec = Some(temp_rec);
                }
                None => (),
            }
        }

        rec
    }

    fn bounding_box(&self) -> Option<&AABB> {
        if self.objects.len() == 0 {
            return None;
        }

        Some(&self.bbox)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::rc::Rc;

    use crate::color::Color;
    use crate::geometry::{Point3, Ray, Sphere, Vec3};
    use crate::material::types::Lambertian;

    struct GenericHittable;
    impl Hittable for GenericHittable {
        fn hit(&self, _ray: &Ray, _t_min: f64, _t_max: f64) -> Option<HitRecord> {
            None
        }
        fn bounding_box(&self) -> Option<&AABB> {
            None
        }
    }

    #[test]
    fn test_hitrecord_new() {
        let mut rec: HitRecord;

        let ray_origin = Vec3::new(0., 0., 0.);
        let ray_opposite_direction = Ray::new(ray_origin.clone(), Vec3::new(0.0, 0.0, -1.0));

        let t = 0.5;
        let p = Vec3::new(0.0, 0.0, -0.5);
        let normal = Vec3::new(0.0, 0.0, 1.0);
        let material_rc = Rc::new(Lambertian::new(Color::new(0., 0., 0.)));

        rec = HitRecord::new(
            &ray_opposite_direction,
            t,
            p.clone(),
            normal.clone(),
            material_rc.clone(),
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

    #[test]
    fn test_hittablelist_new() {
        let world = HittableList::new();
        assert_eq!(world.objects.len(), 0);
    }

    #[test]
    fn test_hittablelist_add() {
        let mut world = HittableList::new();

        world.add(Box::new(GenericHittable {}));
        assert_eq!(world.objects.len(), 1);

        world.add(Box::new(GenericHittable {}));
        world.add(Box::new(GenericHittable {}));
        assert_eq!(world.objects.len(), 3);
    }

    #[test]
    fn test_hittablelist_hit() {
        let mut rec: Option<HitRecord>;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let mut world = HittableList::new();
        let t_min = 0.;
        let t_max = std::f64::INFINITY;

        let r_hit = Ray::new(origin.clone(), Vec3::new(0.0, 0.0, -1.0));
        rec = world.hit(&r_hit, t_min, t_max);
        let miss = rec.is_none();
        assert!(miss);

        let material_rc = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
        let sphere = Sphere::new(Point3::new(0., 0., -1.), 0.5, material_rc);
        world.add(Box::new(sphere));

        rec = world.hit(&r_hit, t_min, t_max);
        let hit = rec.is_some();
        assert!(hit);

        let r_miss = Ray::new(origin.clone(), Vec3::new(1.0, 1.0, 0.0));
        rec = world.hit(&r_miss, t_min, t_max);
        let miss = rec.is_none();
        assert!(miss);
    }
}
