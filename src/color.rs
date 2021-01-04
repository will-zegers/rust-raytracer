use std::fs::File;
use std::io::prelude::*;
use std::ops;

pub struct Color(f64, f64, f64);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color(r, g, b)
    }

    pub fn r(&self) -> f64 {
        self.0
    }

    pub fn g(&self) -> f64 {
        self.1
    }

    pub fn b(&self) -> f64 {
        self.2
    }
}

#[test]
fn test_color_new() {
    let v = Color::new(1.0, 2.0, 3.0);

    assert_eq!(v.0, 1.0);
    assert_eq!(v.1, 2.0);
    assert_eq!(v.2, 3.0);

    assert_eq!(v.r(), v.0);
    assert_eq!(v.g(), v.1);
    assert_eq!(v.b(), v.2);
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

#[test]
fn test_color_ops() {
    let v1 = Color::new(1.0, 2.0, 3.0);
    let v2 = Color::new(3.0, 4.0, 12.0);
    let v_add = v1 + v2;

    assert_eq!(v_add.0, 4.0);
    assert_eq!(v_add.1, 6.0);
    assert_eq!(v_add.2, 15.0);
}

pub fn write_color(file: &mut File, pixel_color: Color) -> std::io::Result<()> {
    let ir = (255.999 * pixel_color.r()) as i32;
    let ig = (255.999 * pixel_color.g()) as i32;
    let ib = (255.999 * pixel_color.b()) as i32;

    let pixel = format!("{} {} {}\n", ir, ig, ib);
    file.write_all(pixel.as_bytes())?;

    Ok(())
}
