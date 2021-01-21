use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::rc::Rc;
use std::str::FromStr;

use rand::Rng;

mod camera;
use camera::{Camera, CameraOrientation, CameraSettings};

mod color;
use color::Color;

mod geometry;
use geometry::{HittableList, Point3, Sphere, Vec3};

mod material;
use material::types::{Dielectric, Lambertian, Metal};
use material::Material;

mod texture;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        print_usage(&args[0]);
        std::process::exit(1);
    }

    let path = Path::new(&args[1]);
    let mut file = File::create(&path).expect("could not open file for writing");

    // Image
    let (image_width, image_height) = match parse_dimensions(&args[2]) {
        Some(dims) => dims,
        None => {
            print_usage(&args[0]);
            std::process::exit(1);
        }
    };
    let samples_per_pixel: i32 = args[3].parse().expect("invalid SAMPLES_PER_PIXEL param");
    let max_depth = args[4].parse().expect("invalid MAX_RAYTRACE_DEPTH param");

    // Camera
    let orientation = CameraOrientation {
        lookfrom: Point3::new(13., 2., 3.),
        lookat: Point3::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
    };
    let settings = CameraSettings {
        vfov: 20.0,
        aspect_ratio: 16.0 / 9.0,
        aperture: 0.1,
        focus_dist: 10.,
    };
    let camera = Camera::new(settings, orientation);

    // World
    let world = random_scene();

    // Render
    // write the PPM header to file
    let header = format!("P3\n{} {}\n255\n", image_width, image_height);
    file.write_all(header.as_bytes())
        .expect("could not write to ppm file");

    let mut rng = rand::thread_rng();
    for j in (0..image_height).rev() {
        print!("Scan lines remaining: {} \r", j);
        io::stdout().flush().unwrap();
        for i in 0..image_width {
            let mut pixel_color = Color::new(0., 0., 0.);
            for _ in 0..samples_per_pixel {
                let u = ((i as f64) + rng.gen::<f64>()) / ((image_width - 1) as f64);
                let v = ((j as f64) + rng.gen::<f64>()) / ((image_height - 1) as f64);
                let r = camera.get_ray(u, v);
                pixel_color += r.color(&world, max_depth);
            }

            let pixel = color::get_pixel(pixel_color, samples_per_pixel);
            file.write_all(pixel.as_bytes())
                .expect("could not write to .ppm file");
        }
    }
}

fn print_usage(name: &str) {
    writeln!(
        std::io::stderr(),
        "Usage: {} FILE DIMENSIONS SAMPLES_PER_PIXEL MAX_RAYTRACE_DEPTH",
        name
    )
    .unwrap();
    writeln!(
        std::io::stderr(),
        "Example: {} ./image.ppm 256x256 100 50",
        name
    )
    .unwrap();
}

/// Parse the string `s` as a coordinate pair, like `"400x600"`.
///
/// Specifically, `s` should have the form <left>x<right>, where <left> and <right> are both
/// strings that can be parsed by `i32::from_str`.
///
/// If `s` has the proper form, return `Some<(x, y)>`. If it doesn't parse correctly, return
/// `None`.
fn parse_dimensions(s: &str) -> Option<(i32, i32)> {
    match s.find('x') {
        None => None,
        Some(index) => match (i32::from_str(&s[..index]), i32::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        ground_material,
    )));

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
                let sphere_material: Rc<dyn Material>;

                let random_material = rng.gen::<f64>();
                if random_material < 0.65 {
                    // diffuse
                    let albedo = Color::random(0., 1.) * Color::random(0., 1.);
                    sphere_material = Rc::new(Lambertian::new(albedo));
                } else if random_material < 0.85 {
                    // metal
                    let albedo = Color::random(0., 1.);
                    let fuzz = rng.gen_range(0.0..0.25);
                    sphere_material = Rc::new(Metal::new(albedo, fuzz));
                } else {
                    // glass
                    sphere_material = Rc::new(Dielectric::new(1.5));
                }
                world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0., 1., 0.),
        1.,
        material1,
    )));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point3::new(-4., 1., 0.),
        1.,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4., 1., 0.),
        1.,
        material3,
    )));

    world
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_dimensions() {
        assert_eq!(parse_dimensions(""), None);
        assert_eq!(parse_dimensions("10x"), None);
        assert_eq!(parse_dimensions("x10"), None);
        assert_eq!(parse_dimensions("10x20"), Some((10, 20)));
        assert_eq!(parse_dimensions("10x20foo"), None);
    }
}
