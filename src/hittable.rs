use std::rc::Rc;

use crate::material::Material;
use crate::ray::Ray;
use crate::vec3;
use crate::vec3::{Point3, Vec3};


#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub material: Rc<dyn Material>,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl HitRecord {
    // pub fn new(&ray: Ray, outward_normal: Vec3) -> HitRecord {
    //     HitRecord {
    //         p: Point3::new(0., 0., 0.),
    //         normal: set_face_normal(&ray, outward_normal),
    //         t: 0.,
    //         material: Rc::new(Lambertian::new(Color::new(0., 0., 0.))),
    //     }
    // }

    pub fn get_face_normal(ray: &Ray, outward_normal: Vec3) -> Vec3 {
        let front_face = vec3::dot(&ray.direction, &outward_normal) < 0.;
        if front_face {
            return outward_normal;
        }

        -outward_normal
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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut rec: Option<HitRecord> = None;

        for object in &self.objects {
            match object.hit(ray, t_min, closest_so_far) {
                Some(temp_rec) => {
                    closest_so_far = temp_rec.t;
                    rec = Some(temp_rec);
                },
                None => (),
            }
        }

        rec
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::rc::Rc;

    use crate::color::Color;
    use crate::material::Lambertian;
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::vec3::{Point3, Vec3};

    struct GenericHittable;
    impl Hittable for GenericHittable {
        fn hit(&self, _ray: &Ray, _t_min: f64, _t_max: f64) -> Option<HitRecord> {
            None
        }
    }

    #[test]
    fn test_hitrecord_set_face_normal() {
        let origin = Point3::new(0., 0., 0.);
        let ray = Ray::new(&origin, Vec3::new(0., 0., 0.5));

        let outward_normal_same = Vec3::new(0., 0., -0.5);
        let normal = HitRecord::get_face_normal(&ray, outward_normal_same.clone());
        assert_eq!(normal, outward_normal_same);

        let outward_normal_opposite = Vec3::new(0., 0., 0.5);
        let normal = HitRecord::get_face_normal(&ray, outward_normal_opposite.clone());
        assert_eq!(-normal, outward_normal_opposite);
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
        let mut rec: Option<HitRecord>;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let mut world = HittableList::new();
        let t_min = 0.;
        let t_max = std::f64::INFINITY;


        let r_hit = Ray::new(&origin, Vec3::new(0.0, 0.0, -1.0));
        rec = world.hit(&r_hit, t_min, t_max);
        let miss = rec.is_none();
        assert!(miss);

        let material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
        let sphere = Sphere::new(Point3::new(0., 0., -1.), 0.5, material);
        world.add(Box::new(sphere));

        rec = world.hit(&r_hit, t_min, t_max);
        let hit = rec.is_some();
        assert!(hit);

        let r_miss = Ray::new(&origin, Vec3::new(1.0, 1.0, 0.0));
        rec = world.hit(&r_miss, t_min, t_max);
        let miss = rec.is_none();
        assert!(miss);
    }
}
