use std::fs::File;
use std::io::prelude::*;
use std::ops;

use rand::Rng;

#[derive(Clone, Debug)]
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

    /// Generate a random color
    pub fn random(min: f64, max: f64) -> Color {
        let mut rng = rand::thread_rng();
        Color(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::AddAssign<Color> for Color {
    fn add_assign(&mut self, other: Color) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

impl ops::Mul<Color> for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Color::new(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
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

pub fn write_color(
    file: &mut File,
    pixel_color: Color,
    samples_per_pixel: i32,
) -> std::io::Result<()> {
    let scale = 1.0 / (samples_per_pixel as f64);

    // sqrt for gamma 2 correction
    let r = f64::sqrt(scale * pixel_color.r());
    let g = f64::sqrt(scale * pixel_color.g());
    let b = f64::sqrt(scale * pixel_color.b());

    let ir = (256. * clamp(r, 0., 0.999)) as i32;
    let ig = (256. * clamp(g, 0., 0.999)) as i32;
    let ib = (256. * clamp(b, 0., 0.999)) as i32;

    let pixel = format!("{} {} {}\n", ir, ig, ib);
    file.write_all(pixel.as_bytes())?;

    Ok(())
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    } else if x > max {
        return max;
    }
    x
}

#[cfg(test)]
mod test {
    use super::Color;

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

    #[test]
    fn test_color_equality() {
        let c1 = Color(1.0, 2.0, 3.0);
        let c2 = Color(1.0, 2.0, 3.0);
        assert_eq!(c1, c2);

        let c3 = Color(1.0, 2.0, 2.0);
        assert_ne!(c1, c3);
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
}
