use std::ops;

use rand::Rng;

#[derive(Clone, Debug, PartialEq)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    /// Generate a random color
    pub fn random(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            r: rng.gen_range(min..max),
            g: rng.gen_range(min..max),
            b: rng.gen_range(min..max),
        }
    }
}

impl ops::Add<Color> for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl ops::AddAssign<Color> for Color {
    fn add_assign(&mut self, other: Self) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}

impl ops::Mul<&Color> for &Color {
    type Output = Color;

    fn mul(self, rhs: &Color) -> Color {
        Color::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Color {
        &self * &rhs
    }
}

impl ops::Mul<&Color> for Color {
    type Output = Color;

    fn mul(self, rhs: &Self) -> Color {
        &self * rhs
    }
}

impl ops::Mul<Color> for &Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        self * &rhs
    }
}

impl ops::Mul<f64> for Color {
    type Output = Self;

    fn mul(self, t: f64) -> Self {
        Color::new(t * self.r, t * self.g, t * self.b)
    }
}

impl ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        rhs * self
    }
}

pub fn get_pixel(pixel_color: Color, samples_per_pixel: i32) -> String {
    let scale = 1.0 / (samples_per_pixel as f64);

    // sqrt for gamma 2 correction
    let r = f64::sqrt(scale * pixel_color.r);
    let g = f64::sqrt(scale * pixel_color.g);
    let b = f64::sqrt(scale * pixel_color.b);

    let ir = (256. * clamp(r, 0., 0.999)) as i32;
    let ig = (256. * clamp(g, 0., 0.999)) as i32;
    let ib = (256. * clamp(b, 0., 0.999)) as i32;

    format!("{} {} {}\n", ir, ig, ib)
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

        assert_eq!(c.r, 1.0);
        assert_eq!(c.g, 2.0);
        assert_eq!(c.b, 3.0);
    }

    #[test]
    fn test_color_equality() {
        let c1 = Color {
            r: 1.0,
            g: 2.0,
            b: 3.0,
        };
        let c2 = Color {
            r: 1.0,
            g: 2.0,
            b: 3.0,
        };
        assert_eq!(c1, c2);

        let c3 = Color {
            r: 1.0,
            g: 2.0,
            b: 2.0,
        };
        assert_ne!(c1, c3);
    }

    #[test]
    fn test_color_ops() {
        // addition
        let c1 = Color::new(1.0, 2.0, 3.0);
        let c2 = Color::new(3.0, 4.0, 12.0);
        let c_add = c1 + c2;
        assert_eq!(
            c_add,
            Color {
                r: 4.,
                g: 6.,
                b: 15.
            }
        );

        // in-place addition
        let mut c3 = Color::new(1.0, 2.0, 3.0);
        c3 += Color::new(3.0, 4.0, 12.0);
        assert_eq!(
            c3,
            Color {
                r: 4.,
                g: 6.,
                b: 15.
            }
        );

        // scalar multiplication
        let c4 = Color::new(1.0, 2.0, 3.0);
        assert_eq!(
            2. * c4.clone(),
            Color {
                r: 2.,
                g: 4.,
                b: 6.
            }
        );
        assert_eq!(2. * c4.clone(), c4.clone() * 2.);

        // color-color multiplication
        let c5 = Color::new(3., 5., 8.);
        assert_eq!(c4 * c5, Color::new(3., 10., 24.));
    }
}
