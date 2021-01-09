use crate::ray::Ray;
use crate::vec3;
use crate::vec3::{Point3, Vec3};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: Point3::new(0., 0., 0.),
            normal: Vec3::new(0., 0., 0.),
            t: 0.,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        let front_face = vec3::dot(&ray.direction, &outward_normal) < 0.;
        if front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    #[allow(dead_code)]
    fn len(&self) -> usize {
        self.objects.len()
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut temp_rec = HitRecord::new();

        for object in &self.objects {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                *rec = temp_rec.clone();
                closest_so_far = rec.t;
            }
        }
        hit_anything
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::vec3::{Point3, Vec3};

    struct GenericHittable;
    impl Hittable for GenericHittable {
        fn hit(&self, _ray: &Ray, _t_min: f64, _t_max: f64, _rec: &mut HitRecord) -> bool {
            true
        }
    }

    #[test]
    fn test_hitrecord_new() {
        let rec = HitRecord::new();

        assert_eq!(rec.p, Point3::new(0., 0., 0.));
        assert_eq!(rec.normal, Vec3::new(0., 0., 0.));
        assert_eq!(rec.t, 0.);
    }

    #[test]
    fn test_hitrecord_set_face_normal() {
        let mut rec = HitRecord::new();
        let origin = Point3::new(0., 0., 0.);
        let ray = Ray::new(&origin, Vec3::new(0., 0., 0.5));

        let outward_normal_same = Vec3::new(0., 0., -0.5);
        rec.set_face_normal(&ray, outward_normal_same.clone());
        assert_eq!(rec.normal, outward_normal_same);

        let outward_normal_opposite = Vec3::new(0., 0., 0.5);
        rec.set_face_normal(&ray, outward_normal_same.clone());
        assert_eq!(-rec.normal, outward_normal_opposite);
    }

    #[test]
    fn test_hittablelist_new() {
        let world = HittableList::new();
        assert_eq!(world.len(), 0);
    }

    #[test]
    fn test_hittablelist_add() {
        let mut world = HittableList::new();

        world.add(Box::new(GenericHittable {}));
        assert_eq!(world.len(), 1);

        world.add(Box::new(GenericHittable {}));
        world.add(Box::new(GenericHittable {}));
        assert_eq!(world.len(), 3);
    }

    #[test]
    fn test_hittablelist_clear() {
        let mut world = HittableList::new();

        world.add(Box::new(GenericHittable {}));
        world.add(Box::new(GenericHittable {}));
        world.add(Box::new(GenericHittable {}));

        world.clear();
        assert_eq!(world.len(), 0);
    }

    #[test]
    fn test_hittablelist_hit() {
        let origin = Point3::new(0.0, 0.0, 0.0);
        let mut world = HittableList::new();
        let mut rec = HitRecord::new();

        let r_hit = Ray::new(&origin, Vec3::new(0.0, 0.0, -1.0));
        let miss = !world.hit(&r_hit, 0., std::f64::INFINITY, &mut rec);
        assert!(miss);

        world.add(Box::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));

        let hit = world.hit(&r_hit, 0., std::f64::INFINITY, &mut rec);
        assert!(hit);

        let r_miss = Ray::new(&origin, Vec3::new(1.0, 1.0, 0.0));
        let miss = !world.hit(&r_miss, 0., std::f64::INFINITY, &mut rec);
        assert!(miss);
    }
}
