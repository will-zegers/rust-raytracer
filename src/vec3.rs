use std::ops;

use rand::Rng;

const TOL: f64 = 1e-8;

#[derive(Clone, Debug)]
pub struct Vec3(f64, f64, f64);
pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3(x, y, z)
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(&self) -> Vec3 {
        return self / self.length();
    }

    pub fn near_zero(&self) -> bool {
        f64::abs(self.0) < TOL && f64::abs(self.1) < TOL && f64::abs(self.2) < TOL
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

impl ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Vec3 {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        &self + &rhs
    }
}

impl ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        self + &rhs
    }
}

impl ops::Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Vec3 {
        &self + rhs
    }
}

impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        (1.0 / rhs) * self
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        &self / rhs
    }
}

impl ops::Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Vec3 {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        self * &rhs
    }
}

// TODO: figure out if there's a way to unify multiple traits for ref and non-ref
impl ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Vec3 {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        self - &rhs
    }
}

impl ops::Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Vec3 {
        &self - rhs
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        &self - &rhs
    }
}

impl ops::Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        -&self
    }
}

/// Compute the dot product of two Vec3s
pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.0 * v.0 + u.1 * v.1 + u.2 * v.2
}

#[allow(dead_code)]
pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let mut in_unit_sphere = random_in_unit_sphere();
    if dot(&in_unit_sphere, &normal) <= 0. {
        //In the same hemisphere as the normal
        in_unit_sphere = -in_unit_sphere;
    }
    in_unit_sphere
}

#[allow(dead_code)]
/// For 'true' Lambertian diffuse scattering
pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().unit_vector()
}

/// Simplified diffuse scattering
pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let v = random(-1., 1.);
        if v.length_squared() >= 1. {
            continue;
        }
        return v;
    }
}

/// Generate a random 3-vector
fn random(min: f64, max: f64) -> Vec3 {
    let mut rng = rand::thread_rng();
    Vec3(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vec3_new() {
        let v = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(v.0, 1.0);
        assert_eq!(v.1, 2.0);
        assert_eq!(v.2, 3.0);

        assert_eq!(v.x(), v.0);
        assert_eq!(v.y(), v.1);
        assert_eq!(v.z(), v.2);
    }

    #[test]
    fn test_vec3_length() {
        let v = Vec3::new(3.0, 4.0, 12.0);

        assert_eq!(v.length_squared(), 169.0);
        assert_eq!(v.length(), 13.0);
    }

    #[test]
    fn test_vec3_unit_vector() {
        let v = Vec3::new(1.0, 2.0, 2.0);
        let u = v.unit_vector();

        assert_eq!(u, Vec3(1.0 / 3.0, 2.0 / 3.0, 2.0 / 3.0));
    }

    #[test]
    fn test_vec3_equality() {
        let v1 = Vec3(1.0, 2.0, 3.0);
        let v2 = Vec3(1.0, 2.0, 3.0);
        assert_eq!(v1, v2);

        let v3 = Vec3(1.0, 2.0, 2.0);
        assert_ne!(v1, v3);
    }

    #[test]
    fn test_vec3_ops() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(3.0, 4.0, 12.0);
        let v_add = v1 + v2;
        assert_eq!(v_add, Vec3(4.0, 6.0, 15.0));

        let v3 = Vec3::new(1.0, 2.0, 3.0);
        let v_div = &v3 / 2.0;
        assert_eq!(v_div, Vec3(0.5, 1.0, 1.5));

        let v4 = Vec3::new(0.125, 0.25, 0.5);
        let v_mul = 8.0 * &v4;
        assert_eq!(v_mul, Vec3(1.0, 2.0, 4.0));

        let v8 = Vec3::new(0.125, 0.25, 0.5);
        let v_mul = 8.0 * v8;
        assert_eq!(v_mul, Vec3(1.0, 2.0, 4.0));

        let v5 = Vec3::new(9.0, 8.0, 7.0);
        let v6 = Vec3::new(3.0, 2.0, 1.0);
        let v_sub = v5 - v6;
        assert_eq!(v_sub, Vec3(6.0, 6.0, 6.0));

        let v7 = Vec3::new(1.0, 2.0, 3.0);
        let v_neg = -v7;
        assert_eq!(v_neg, Vec3(-1.0, -2.0, -3.0));
    }

    #[test]
    fn test_vec3_dot() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(3.0, 4.0, 12.0);

        let res = dot(&v1, &v2);
        assert_eq!(res, 47.0);
    }

    #[test]
    fn test_vec3_random() {
        let min = -1.;
        let max = 1.;

        let v1 = random(min, max);
        let v2 = random(min, max);
        assert_ne!(v1, v2);
    }

    #[test]
    fn test_vec3_random_in_unit_sphere() {
        let v = random_in_unit_sphere();
        assert!(v.length_squared() < 1.)
    }

    #[test]
    fn test_vec3_random_unit_vector() {
        let v = random_unit_vector();
        assert!(f64::abs(1.0 - v.length()) < TOL);
    }

    #[test]
    fn test_vec3_random_in_hemisphere() {
        let normal = Vec3(0., 0., -1.);
        let in_unit_sphere = random_in_hemisphere(&normal);
        assert!(dot(&in_unit_sphere, &normal) > 0.)
    }
}
