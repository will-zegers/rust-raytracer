use std::ops::{Add, Div, Index, Mul, Neg, Sub};

use rand::Rng;

#[derive(Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
pub type Point3 = Vec3;

pub enum RandomVectorType {
    InUnitDisk,
    InUnitSphere,
    Unit,
}

impl Vec3 {
    pub const TOL: f64 = 1e-8;

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn random(vector_type: RandomVectorType) -> Vec3 {
        match vector_type {
            RandomVectorType::InUnitDisk => random_in_unit_disk(),
            RandomVectorType::InUnitSphere => random_in_unit_sphere(),
            RandomVectorType::Unit => random_unit_vector(),
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(&self) -> Vec3 {
        return self / self.length();
    }

    pub fn near_zero(&self) -> bool {
        f64::abs(self.x) < Self::TOL && f64::abs(self.y) < Self::TOL && f64::abs(self.z) < Self::TOL
    }

    pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3 {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        (self - other).near_zero()
    }
}

impl Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        &self + &rhs
    }
}

impl Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        self + &rhs
    }
}

impl Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Vec3 {
        &self + rhs
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        (1.0 / rhs) * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        &self / rhs
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &f64 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("invalid vector index: {:?}", i),
        }
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        self * &rhs
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        rhs * &self
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        rhs * self
    }
}

// TODO: figure out if there's a way to unify multiple traits for ref and non-ref
impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        self - &rhs
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Vec3 {
        &self - rhs
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        &self - &rhs
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        -&self
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let v = Vec3 {
            x: rng.gen_range(-1.0..1.0),
            y: rng.gen_range(-1.0..1.0),
            z: rng.gen_range(-1.0..1.0),
        };
        if v.length_squared() >= 1. {
            continue;
        }
        return v;
    }
}

fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let v = Vec3 {
            x: rng.gen_range(-1.0..1.0),
            y: rng.gen_range(-1.0..1.0),
            z: 0.0,
        };
        if v.length_squared() < 1. {
            return v;
        }
    }
}

fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().unit_vector()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vec3_new() {
        let v = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_vec3_length() {
        let v = Vec3 {
            x: 3.0,
            y: 4.0,
            z: 12.0,
        };

        assert_eq!(v.length_squared(), 169.0);
        assert_eq!(v.length(), 13.0);
    }

    #[test]
    fn test_vec3_unit_vector() {
        let v = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 2.0,
        };
        let u = v.unit_vector();

        assert_eq!(
            u,
            Vec3 {
                x: 1.0 / 3.0,
                y: 2.0 / 3.0,
                z: 2.0 / 3.0
            }
        );
    }

    #[test]
    fn test_vec3_equality() {
        let v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        assert_eq!(v1, v2);

        let v3 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 2.0,
        };
        assert_ne!(v1, v3);

        let v4 = Vec3 {
            x: v1.x - (Vec3::TOL / 2.),
            y: v1.y + (Vec3::TOL / 2.),
            z: v1.z,
        };
        assert_eq!(v1, v4);

        let v5 = Vec3 {
            x: v1.x - 2. * Vec3::TOL,
            y: v1.y + 2. * Vec3::TOL,
            z: v1.z,
        };
        assert_ne!(v1, v5);
    }

    #[test]
    fn test_vec3_ops() {
        // addition
        let v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = Vec3 {
            x: 3.0,
            y: 4.0,
            z: 12.0,
        };
        let v_add = v1 + v2;
        assert_eq!(
            v_add,
            Vec3 {
                x: 4.0,
                y: 6.0,
                z: 15.0
            }
        );

        // division
        let v3 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v_div = &v3 / 2.0;
        assert_eq!(
            v_div,
            Vec3 {
                x: 0.5,
                y: 1.0,
                z: 1.5
            }
        );

        // multiplication
        let v4 = Vec3 {
            x: 0.125,
            y: 0.25,
            z: 0.5,
        };
        let v_mul = 8.0 * &v4;
        assert_eq!(
            v_mul,
            Vec3 {
                x: 1.0,
                y: 2.0,
                z: 4.0
            }
        );

        let v8 = Vec3 {
            x: 0.125,
            y: 0.25,
            z: 0.5,
        };
        let v_mul = 8.0 * v8;
        assert_eq!(
            v_mul,
            Vec3 {
                x: 1.0,
                y: 2.0,
                z: 4.0
            }
        );

        // subtraction
        let v5 = Vec3 {
            x: 9.0,
            y: 8.0,
            z: 7.0,
        };
        let v6 = Vec3 {
            x: 3.0,
            y: 2.0,
            z: 1.0,
        };
        let v_sub = v5 - v6;
        assert_eq!(
            v_sub,
            Vec3 {
                x: 6.0,
                y: 6.0,
                z: 6.0
            }
        );

        // negation
        let v7 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v_neg = -v7;
        assert_eq!(
            v_neg,
            Vec3 {
                x: -1.0,
                y: -2.0,
                z: -3.0
            }
        );

        // indexing
        let v8 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        assert_eq!(v8[0], 1.0);
        assert_eq!(v8[1], 2.0);
        assert_eq!(v8[2], 3.0);
    }

    #[test]
    fn test_vec3_dot() {
        let v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = Vec3 {
            x: 3.0,
            y: 4.0,
            z: 12.0,
        };

        let res = Vec3::dot(&v1, &v2);
        assert_eq!(res, 47.0);
    }

    #[test]
    fn test_vec3_random_in_unit_sphere() {
        let v = random_in_unit_sphere();
        assert!(v.length_squared() < 1.)
    }
}
