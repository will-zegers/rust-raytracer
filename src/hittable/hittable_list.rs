use super::{AABB, HitRecord, Hittable};
use crate::geometry::{Ray, Point3};

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
    use crate::geometry::{Sphere, Vec3};
    use crate::hittable::HitRecord;
    use crate::material::types::Lambertian;
    use crate::texture::SolidColor;

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

        let color = Rc::new(SolidColor {
            color: Color::new(0.5, 0.5, 0.5),
        });
        let material_rc = Rc::new(Lambertian::new(color));
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

    #[test]
    fn test_hittablelist_bounding_box() {
        let mut world = HittableList::new();
        assert!(world.bounding_box().is_none());

        let color = Rc::new(SolidColor {
            color: Color::new(0.5, 0.5, 0.5),
        });
        let material_rc = Rc::new(Lambertian::new(color));
        let sphere1 = Sphere::new(Point3::new(0., 0., -1.), 0.5, material_rc.clone());
        world.add(Box::new(sphere1));

        assert!(world.bounding_box().is_some());
        assert!(
            *world.bounding_box().unwrap()
                == AABB {
                    minimum: Vec3 {
                        x: -0.5,
                        y: -0.5,
                        z: -1.5
                    },
                    maximum: Vec3 {
                        x: 0.5,
                        y: 0.5,
                        z: -0.5
                    }
                }
        );

        let sphere2 = Sphere::new(Point3::new(1., 0., -1.), 0.5, material_rc.clone());
        world.add(Box::new(sphere2));
        assert_eq!(
            *world.bounding_box().unwrap(),
            AABB {
                minimum: Vec3::new(-0.5, -0.5, -1.5),
                maximum: Vec3::new(1.5, 0.5, -0.5)
            }
        );

        let sphere3 = Sphere::new(Point3::new(0., 1., 0.), 0.5, material_rc.clone());
        world.add(Box::new(sphere3));
        assert_eq!(
            *world.bounding_box().unwrap(),
            AABB {
                minimum: Vec3::new(-0.5, -0.5, -1.5),
                maximum: Vec3::new(1.5, 1.5, 0.5)
            }
        );

        let sphere4 = Sphere::new(Point3::new(-1., -1., 0.), 0.5, material_rc.clone());
        world.add(Box::new(sphere4));
        assert_eq!(
            *world.bounding_box().unwrap(),
            AABB {
                minimum: Vec3::new(-1.5, -1.5, -1.5),
                maximum: Vec3::new(1.5, 1.5, 0.5)
            }
        );
    }
}
