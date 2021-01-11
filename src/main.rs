use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::rc::Rc;
use std::str::FromStr;

use rand::Rng;

mod camera;
use crate::camera::Camera;

mod color;
use crate::color::Color;

mod hittable;
use crate::hittable::HittableList;

mod material;
use crate::material::{Lambertian, Metal};

mod ray;

mod sphere;
use crate::sphere::Sphere;

mod vec3;
use crate::vec3::Point3;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
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
    let aspect_ratio = image_width as f64 / image_height as f64;
    let samples_per_pixel = 10;
    let max_depth = 10;

    // Camera
    let camera = Camera::new(aspect_ratio);

    // World
    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left   = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right  = Rc::new(Metal::new(Color::new(0.6, 0.6, 0.2), 0.0));

    // add some objects to our world for rays to intersect with.
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(
        Point3::new(0., 0., -1.),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0., -1.),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0., -1.),
        0.5,
        material_right,
    )));

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

            color::write_color(&mut file, pixel_color, samples_per_pixel)
                .expect("could not write to ppm file");
        }
    }
}

fn print_usage(name: &str) {
    writeln!(std::io::stderr(), "Usage: {} FILE DIMENSIONS", name).unwrap();
    writeln!(std::io::stderr(), "Example: {} ./image.ppm 256x256", name).unwrap();
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
