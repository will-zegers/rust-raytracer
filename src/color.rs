use std::fs::File;
use std::io::prelude::*;
use std::ops;

#[derive(Debug)]
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
    let c = Color::new(1.0, 2.0, 3.0);

    assert_eq!(c.0, 1.0);
    assert_eq!(c.1, 2.0);
    assert_eq!(c.2, 3.0);

    assert_eq!(c.r(), c.0);
    assert_eq!(c.g(), c.1);
    assert_eq!(c.b(), c.2);
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

#[test]
fn test_color_equality() {
    let c1 = Color(1.0, 2.0, 3.0);
    let c2 = Color(1.0, 2.0, 3.0);
    assert_eq!(c1, c2);

    let c3 = Color(1.0, 2.0, 2.0);
    assert_ne!(c1, c3);
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, t: f64) -> Color {
        Color::new(t * self.r(), t * self.g(), t * self.b())
    }
}

impl ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        rhs * self
    }
}

#[test]
fn test_color_ops() {
    let c1 = Color::new(1.0, 2.0, 3.0);
    let c2 = Color::new(3.0, 4.0, 12.0);
    let c_add = c1 + c2;

    assert_eq!(c_add.0, 4.0);
    assert_eq!(c_add.1, 6.0);
    assert_eq!(c_add.2, 15.0);
}

pub fn write_color(file: &mut File, pixel_color: Color) -> std::io::Result<()> {
    let ir = (255.999 * pixel_color.r()) as i32;
    let ig = (255.999 * pixel_color.g()) as i32;
    let ib = (255.999 * pixel_color.b()) as i32;

    let pixel = format!("{} {} {}\n", ir, ig, ib);
    file.write_all(pixel.as_bytes())?;

    Ok(())
}
