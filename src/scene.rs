use std::rc::Rc;

use rand::Rng;

use crate::color::Color;

use crate::geometry::{
    AxisAlignment, Block, ConstantMedium, Point3, Rect, RectCorner, Sphere, Vec3,
};

use crate::hittable::instance::{Rotate, Translate};
use crate::hittable::{BVHNode, Hittable, HittableList};

use crate::material::types::{Dielectric, DiffuseLight, Lambertian, Metal};
use crate::material::Material;

use crate::texture::{Checker, ImageTexture, Noise, NoiseStrategy, SolidColor};

pub struct RandomScene;

impl RandomScene {
    pub fn new() -> HittableList {
        let mut world = HittableList::new();

        let ground_texture = Checker {
            odd: Rc::new(SolidColor {
                color: Color::new(0.1, 0.1, 0.1),
            }),
            even: Rc::new(SolidColor {
                color: Color::new(0.9, 0.9, 0.9),
            }),
        };
        let ground_material = Lambertian::new(Rc::new(ground_texture));
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
                        let albedo = Rc::new(SolidColor {
                            color: Color::random(0., 1.) * Color::random(0., 1.),
                        });
                        material = Rc::new(Lambertian::new(albedo));
                    } else if random_material < 0.85 {
                        // metal
                        let albedo = Rc::new(SolidColor {
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

        let color2 = Rc::new(SolidColor {
            color: Color::new(0.4, 0.2, 0.1),
        });
        let material2 = Rc::new(Lambertian::new(color2));
        let sphere2 = Box::new(Sphere::new(Point3::new(-4., 1., 0.), 1., material2));
        world.add(sphere2);

        let color3 = Rc::new(SolidColor {
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

        let texture = Rc::new(Noise::new(NoiseStrategy::PerlinInterpolation, 4.));
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

pub struct Earth;

impl Earth {
    pub fn new() -> HittableList {
        let texture = Rc::new(ImageTexture::new());
        let surface = Rc::new(Lambertian::new(texture));
        let globe = Box::new(Sphere::new(Point3::new(0., 0., 0.), 2., surface));

        let mut world = HittableList::new();
        world.add(globe);

        world
    }
}

pub struct SimpleLight;

impl SimpleLight {
    pub fn new() -> HittableList {
        let mut world = PerlinSpheres::new();

        let light_color = Color::new(4., 2., 2.);
        let diffuse_light = Rc::new(DiffuseLight::new(light_color));

        let light_rect2 = Box::new(Rect::new(
            AxisAlignment::YZ,
            RectCorner(3., 1.),
            RectCorner(5., 3.),
            -2.,
            diffuse_light,
        ));
        world.add(light_rect2);

        world
    }
}

pub struct SimpleColoredLights;

impl SimpleColoredLights {
    pub fn new() -> HittableList {
        let mut world = PerlinSpheres::new();

        let light_color1 = Color::new(4., 0.5, 0.5);
        let diffuse_light1 = Rc::new(DiffuseLight::new(light_color1));
        let sphere1 = Box::new(Sphere::new(Point3::new(3., 5., 3.5), 1., diffuse_light1));
        world.add(sphere1);

        let light_color2 = Color::new(0.5, 0.5, 4.);
        let diffuse_light2 = Rc::new(DiffuseLight::new(light_color2));
        let sphere2 = Box::new(Sphere::new(Point3::new(4., 3., -3.5), 1., diffuse_light2));
        world.add(sphere2);

        let light_color3 = Color::new(0.5, 4., 0.5);
        let diffuse_light3 = Rc::new(DiffuseLight::new(light_color3));
        let sphere3 = Box::new(Sphere::new(Point3::new(6., 1., 0.), 1., diffuse_light3));
        world.add(sphere3);

        world
    }
}

struct CornellBoxBase;

impl CornellBoxBase {
    pub fn new() -> HittableList {
        let mut world = HittableList::new();

        let red = Rc::new(Lambertian::new(Rc::new(SolidColor {
            color: Color::new(0.65, 0.05, 0.05),
        })));
        world.add(Box::new(Rect::new(
            AxisAlignment::YZ,
            RectCorner(555., 555.),
            RectCorner(0., 0.),
            0.,
            red,
        )));

        let green = Rc::new(Lambertian::new(Rc::new(SolidColor {
            color: Color::new(0.12, 0.45, 0.15),
        })));
        world.add(Box::new(Rect::new(
            AxisAlignment::YZ,
            RectCorner(555., 555.),
            RectCorner(0., 0.),
            555.,
            green,
        )));

        let white = Rc::new(Lambertian::new(Rc::new(SolidColor {
            color: Color::new(0.73, 0.73, 0.73),
        })));
        world.add(Box::new(Rect::new(
            AxisAlignment::XZ,
            RectCorner(555., 555.),
            RectCorner(0., 0.),
            0.,
            white.clone(),
        )));
        world.add(Box::new(Rect::new(
            AxisAlignment::XZ,
            RectCorner(555., 555.),
            RectCorner(0., 0.),
            555.,
            white.clone(),
        )));
        world.add(Box::new(Rect::new(
            AxisAlignment::XY,
            RectCorner(555., 555.),
            RectCorner(0., 0.),
            555.,
            white.clone(),
        )));

        let light = Rc::new(DiffuseLight::new(Color::new(15., 15., 15.)));
        world.add(Box::new(Rect::new(
            AxisAlignment::XZ,
            RectCorner(343., 332.),
            RectCorner(213., 227.),
            554.,
            light,
        )));

        world
    }
}

pub struct CornellBox;

impl CornellBox {
    pub fn new() -> HittableList {
        let mut world = CornellBoxBase::new();

        let white = Rc::new(Lambertian::new(Rc::new(SolidColor {
            color: Color::new(0.73, 0.73, 0.73),
        })));
        let box1 = Box::new(Block::new(
            Point3::new(0., 0., 0.),
            Point3::new(165., 165., 165.),
            white.clone(),
        ));
        let box1_r = Box::new(Rotate::new(box1, -18.));
        let box1_rt = Box::new(Translate::new(box1_r, Vec3::new(130., 0., 65.)));
        world.add(box1_rt);

        let box2 = Box::new(Block::new(
            Point3::new(0., 0., 0.),
            Point3::new(165., 330., 165.),
            white.clone(),
        ));
        let box2_r = Box::new(Rotate::new(box2, 15.));
        let box2_rt = Box::new(Translate::new(box2_r, Vec3::new(265., 0., 295.)));
        world.add(box2_rt);

        world
    }
}

pub struct CornellSmoke;

impl CornellSmoke {
    pub fn new() -> HittableList {
        let mut world = CornellBoxBase::new();

        let white = Rc::new(Lambertian::new(Rc::new(SolidColor {
            color: Color::new(0.73, 0.73, 0.73),
        })));
        let box1 = Box::new(Block::new(
            Point3::new(0., 0., 0.),
            Point3::new(165., 165., 165.),
            white.clone(),
        ));
        let box1_r = Box::new(Rotate::new(box1, -18.));
        let box1_rt = Box::new(Translate::new(box1_r, Vec3::new(130., 0., 65.)));
        let box1_cm = Box::new(ConstantMedium::from_color(
            box1_rt,
            0.01,
            Color::new(1., 1., 1.),
        ));
        world.add(box1_cm);

        let box2 = Box::new(Block::new(
            Point3::new(0., 0., 0.),
            Point3::new(165., 330., 165.),
            white.clone(),
        ));
        let box2_r = Box::new(Rotate::new(box2, 15.));
        let box2_rt = Box::new(Translate::new(box2_r, Vec3::new(265., 0., 295.)));
        let box2_cm = Box::new(ConstantMedium::from_color(
            box2_rt,
            0.01,
            Color::new(0., 0., 0.),
        ));
        world.add(box2_cm);

        world
    }
}

pub struct FinalScene;

impl FinalScene {
    pub fn new() -> HittableList {
        let mut rng = rand::thread_rng();

        let ground = Rc::new(Lambertian::from_color(Color::new(0.48, 0.83, 0.53)));

        let boxes_per_side = 20;
        let mut boxes = Vec::<Rc<dyn Hittable>>::with_capacity(boxes_per_side);
        for i in 0..boxes_per_side {
            let i = i as f64;
            for j in 0..boxes_per_side {
                let j = j as f64;

                let w = 100.;
                let x0 = -1000. + i * w;
                let z0 = -1000. + j * w;
                let y0 = 0.;
                let x1 = x0 + w;
                let y1 = rng.gen_range(1. ..101.);
                let z1 = z0 + w;

                let block = Block::new(
                    Point3::new(x0, y0, z0),
                    Point3::new(x1, y1, z1),
                    ground.clone(),
                );
                boxes.push(Rc::new(block));
            }
        }

        let mut world = HittableList::new();
        world.add(Box::new(BVHNode::new(&boxes, 0, boxes.len())));

        let light = DiffuseLight::new(Color::new(7., 7., 7.));
        let rect_light = Rect::new(
            AxisAlignment::XZ,
            RectCorner(423., 412.),
            RectCorner(123., 147.),
            554.,
            Rc::new(light),
        );
        world.add(Box::new(rect_light));

        let radius_chrome = 50.;
        let center_chrome = Point3::new(400., 400., 400.);
        let material_chrome = Metal::new(
            Rc::new(SolidColor {
                color: Color::new(0.8, 0.15, 0.15),
            }),
            0.0,
        );
        let sphere_chrome = Sphere::new(center_chrome, radius_chrome, Rc::new(material_chrome));
        world.add(Box::new(sphere_chrome));

        let radius_glass = 50.;
        let center_glass = Point3::new(260., 150., 45.);
        let material_glass = Dielectric::new(1.5);
        let material_glass_ptr = Rc::new(material_glass);
        let sphere_glass = Sphere::new(center_glass, radius_glass, material_glass_ptr.clone());
        world.add(Box::new(sphere_glass));

        let radius_brushed = 50.;
        let center_brushed = Point3::new(0., 150., 145.);
        let material_brushed = Metal::new(
            Rc::new(SolidColor {
                color: Color::new(0.8, 0.8, 0.9),
            }),
            1.0,
        );
        let sphere_brushed = Sphere::new(center_brushed, radius_brushed, Rc::new(material_brushed));
        world.add(Box::new(sphere_brushed));

        let radius_boundary = 70.;
        let center_boundary = Point3::new(360., 150., 145.);
        let sphere_boundary =
            Sphere::new(center_boundary, radius_boundary, material_glass_ptr.clone());
        world.add(Box::new(sphere_boundary.clone()));

        let medium =
            ConstantMedium::from_color(Box::new(sphere_boundary), 0.2, Color::new(0.2, 0.4, 0.9));
        world.add(Box::new(medium));

        let radius_boundary2 = 5000.;
        let center_boundary2 = Point3::new(0., 0., 0.);
        let sphere_boundary2 = Sphere::new(
            center_boundary2,
            radius_boundary2,
            material_glass_ptr.clone(),
        );
        let medium2 =
            ConstantMedium::from_color(Box::new(sphere_boundary2), 0.0001, Color::new(1., 1., 1.));
        world.add(Box::new(medium2));

        let radius_earth = 100.;
        let center_earth = Point3::new(400., 200., 400.);
        let texture_earth = ImageTexture::new();
        let material_earth = Lambertian::new(Rc::new(texture_earth));
        let sphere_earth = Sphere::new(center_earth, radius_earth, Rc::new(material_earth));
        world.add(Box::new(sphere_earth));

        world.add(Box::new(Sphere::new(
            Point3::new(220., 280., 300.),
            80.,
            Rc::new(Lambertian::new(Rc::new(Noise::new(
                NoiseStrategy::PerlinInterpolation,
                0.1,
            )))),
        )));

        let mut boxes2 = Vec::<Rc<dyn Hittable>>::with_capacity(1000);
        let white = Rc::new(Lambertian::new(Rc::new(SolidColor {
            color: Color::new(0.73, 0.73, 0.73),
        })));
        for _ in 0..1000 {
            boxes2.push(Rc::new(Sphere::new(
                Point3::new(
                    rng.gen_range(1. ..165.),
                    rng.gen_range(1. ..165.),
                    rng.gen_range(1. ..165.),
                ),
                10.,
                white.clone(),
            )));
        }
        world.add(Box::new(Translate::new(
            Box::new(Rotate::new(Box::new(BVHNode::new(&boxes2, 0, boxes2.len())), 15.)),
            Vec3::new(-100., 270., 395.),
        )));

        world
    }
}
