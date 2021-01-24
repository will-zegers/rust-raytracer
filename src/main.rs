use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::str::FromStr;

use rand::Rng;

mod camera;
use camera::{Camera, CameraOrientation, CameraSettings};

mod color;
use color::Color;

mod geometry;
use geometry::{Point3, Vec3};

mod material;

mod scene;
use scene::{Earth, PerlinSpheres, RandomScene};

mod texture;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 5 {
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

    // World
    let scene = 0;
    let (aperture, background, world) = match scene {
        1 => (
            0.0,
            Color::new(0.7, 0.8, 1.),
            PerlinSpheres::new()
        ),
        2 => (
            0.0,
            Color::new(0.7, 0.8, 1.),
            Earth::new()
        ),
        3 => (
            0.0,
            Color::new(0., 0., 0.),
            Earth::new(),
        ),
        _ => (
            0.0,
            Color::new(0.7, 0.8, 1.),
            RandomScene::new()
        ),
    };

    // Camera
    let orientation = CameraOrientation {
        lookfrom: Point3::new(13., 2., 3.),
        lookat: Point3::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
    };
    let settings = CameraSettings {
        vfov: 20.0,
        aspect_ratio: 16.0 / 9.0,
        aperture,
        focus_dist: 10.,
    };
    let camera = Camera::new(settings, orientation);

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
                pixel_color += r.color(&world, max_depth, &background);
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_dimensions() {
        assert_eq!(parse_dimensions(""), None);
        assert_eq!(parse_dimensions("10x"), None);
        assert_eq!(parse_dimensions("x10"), None);
        assert_eq!(parse_dimensions("10x20foo"), None);
        assert_eq!(parse_dimensions("10x20"), Some((10, 20)));
    }
}
