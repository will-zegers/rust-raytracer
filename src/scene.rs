use std::rc::Rc;

use rand::Rng;

use crate::color::Color;

use crate::geometry::{HittableList, Point3, Sphere};

use crate::material::types::{Dielectric, Lambertian, Metal};
use crate::material::Material;

use crate::texture::{Checker, ImageTexture, Noise, NoiseStrategy, SolidColor};

pub struct RandomScene;

impl RandomScene {
    pub fn new() -> HittableList {
        let mut world = HittableList::new();

        let ground_texture = Checker {
            odd: Box::new(SolidColor {
                color: Color::new(0.1, 0.1, 0.1),
            }),
            even: Box::new(SolidColor {
                color: Color::new(0.9, 0.9, 0.9),
            }),
        };
        let ground_material = Lambertian::new(Box::new(ground_texture));
        let ground_sphere =
            Sphere::new(Point3::new(0., -1000., 0.), 1000., Rc::new(ground_material));
        world.add(Box::new(ground_sphere));

        let mut rng = rand::thread_rng();
        let ref_point = Point3::new(4.0, 0.2, 0.);

        for a in -11..11 {
            for b in -11..11 {
                let center = Point3::new(
                    (a as f64) + 0.9 * rng.gen::<f64>(),
                    0.2,
                    (b as f64) + 0.9 * rng.gen::<f64>(),
                );

                if (&center - &ref_point).length() > 0.9 {
                    let material: Rc<dyn Material>;

                    let random_material = rng.gen::<f64>();
                    if random_material < 0.65 {
                        // diffuse
                        let albedo = Box::new(SolidColor {
                            color: Color::random(0., 1.) * Color::random(0., 1.),
                        });
                        material = Rc::new(Lambertian::new(albedo));
                    } else if random_material < 0.85 {
                        // metal
                        let albedo = Box::new(SolidColor {
                            color: Color::random(0., 1.),
                        });
                        let fuzz = rng.gen_range(0.0..0.25);
                        material = Rc::new(Metal::new(albedo, fuzz));
                    } else {
                        // glass
                        material = Rc::new(Dielectric::new(1.5));
                    }
                    let sphere = Box::new(Sphere::new(center, 0.2, material));
                    world.add(sphere);
                }
            }
        }

        let material1 = Rc::new(Dielectric::new(1.5));
        let sphere1 = Box::new(Sphere::new(Point3::new(0., 1., 0.), 1., material1));
        world.add(sphere1);

        let color2 = Box::new(SolidColor {
            color: Color::new(0.4, 0.2, 0.1),
        });
        let material2 = Rc::new(Lambertian::new(color2));
        let sphere2 = Box::new(Sphere::new(Point3::new(-4., 1., 0.), 1., material2));
        world.add(sphere2);

        let color3 = Box::new(SolidColor {
            color: Color::new(0.7, 0.6, 0.5),
        });
        let material3 = Rc::new(Metal::new(color3, 0.0));
        let sphere3 = Box::new(Sphere::new(Point3::new(4., 1., 0.), 1., material3));
        world.add(sphere3);

        world
    }
}

pub struct PerlinSpheres;

impl PerlinSpheres {
    pub fn new() -> HittableList {
        let mut world = HittableList::new();

        let texture = Box::new(Noise::new(NoiseStrategy::PerlinInterpolation, 4.));
        let material = Rc::new(Lambertian::new(texture));

        let sphere1 = Box::new(Sphere::new(
            Point3::new(0., -1000., 0.),
            1000.,
            material.clone(),
        ));
        world.add(sphere1);

        let sphere2 = Box::new(Sphere::new(Point3::new(0., 2., 0.), 2., material.clone()));
        world.add(sphere2);

        world
    }
}

pub struct Earth();

impl Earth {
    pub fn new() -> HittableList {
        let texture = Box::new(ImageTexture::new());
        let surface = Rc::new(Lambertian::new(texture));
        let globe = Box::new(Sphere::new(Point3::new(0., 0., 0.), 2., surface));

        let mut world = HittableList::new();
        world.add(globe);

        world
    }
}
