use std::ops;

#[derive(Debug)]
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
}

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

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

#[test]
fn test_vec3_equality() {
    let v1 = Vec3(1.0, 2.0, 3.0);
    let v2 = Vec3(1.0, 2.0, 3.0);
    assert_eq!(v1, v2);

    let v3 = Vec3(1.0, 2.0, 2.0);
    assert_ne!(v1, v3);
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

impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        (1.0 / rhs) * self
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

/// Compute the dot product of two Vec3s
#[allow(dead_code)]
pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.0 * v.0 + u.1 * v.1 + u.2 * v.2
}

#[test]
fn test_vec3_dot() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(3.0, 4.0, 12.0);

    let res = dot(&v1, &v2);
    assert_eq!(res, 47.0);
}
