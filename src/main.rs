use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        print_usage(&args[0]);
        std::process::exit(1);
    }

    let path = Path::new(&args[1]);
    let mut file = match File::create(&path) {
        Err(why) => panic!("Could not create file {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let (image_width, image_height) = match parse_dimensions(&args[2]) {
        Some(dims) => dims,
        None => {
            writeln!(std::io::stderr(), "Error parsing dimensions.").unwrap();
            writeln!(std::io::stderr(), "").unwrap();
            print_usage(&args[0]);
            std::process::exit(1);
        },
    };

    let header = format!("P3\n{} {}\n255\n", image_width, image_height);
    file.write_all(header.as_bytes())
        .expect("Error writing to file");

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            let pixel = format!("{} {} {}\n", ir, ig, ib);
            file.write_all(pixel.as_bytes())
                .expect("Error writing to file");
        }
    }
}

fn print_usage(name: &str) {
    writeln!(std::io::stderr(), "Usage: {} FILE DIMENSIONS", name).unwrap();
    writeln!(
        std::io::stderr(),
        "Example: {} ./image.ppm 256x256",
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

#[test]
fn test_parse_dimensions() {
    assert_eq!(parse_dimensions(""), None);
    assert_eq!(parse_dimensions("10x"), None);
    assert_eq!(parse_dimensions("x10"), None);
    assert_eq!(parse_dimensions("10x20"), Some((10, 20)));
    assert_eq!(parse_dimensions("10x20foo"), None);
}
