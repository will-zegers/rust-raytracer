use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::str::FromStr;

mod color;

mod ray;
use crate::ray::Ray;

mod vec3;
use crate::vec3::{Point3, Vec3};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        print_usage(&args[0]);
        std::process::exit(1);
    }

    let path = Path::new(&args[1]);
    let mut file = File::create(&path).expect("could not open file for writing");

    // Image specs
    let (image_width, image_height) = match parse_dimensions(&args[2]) {
        Some(dims) => dims,
        None => {
            print_usage(&args[0]);
            std::process::exit(1);
        }
    };
    let aspect_ratio = (image_width / image_height) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = (aspect_ratio as f64) * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        &origin - &horizontal / 2.0 - &vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let header = format!("P3\n{} {}\n255\n", image_width, image_height);
    file.write_all(header.as_bytes()).expect("could not write to ppm file");

    for j in (0..image_height).rev() {
        print!("Scan lines remaining: {}\r", j);
        io::stdout().flush().unwrap();
        for i in 0..image_width {
            let u = (i as f64) / (image_width - 1) as f64;
            let v = (j as f64) / (image_height - 1) as f64;

            let direction = &lower_left_corner + u * &horizontal + v * &vertical - &origin;
            let r = Ray::new(&origin, direction);
            let pixel_color = r.color();

            color::write_color(&mut file, pixel_color).expect("could not write to ppm file");
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

#[test]
fn test_parse_dimensions() {
    assert_eq!(parse_dimensions(""), None);
    assert_eq!(parse_dimensions("10x"), None);
    assert_eq!(parse_dimensions("x10"), None);
    assert_eq!(parse_dimensions("10x20"), Some((10, 20)));
    assert_eq!(parse_dimensions("10x20foo"), None);
}
