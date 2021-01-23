use rand::seq::SliceRandom;

use crate::geometry::{Point3, RandomVectorType, Vec3};

pub struct Perlin {
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
    random_uvec: Vec<Vec3>,

    pub noise: fn(&Self, &Point3) -> f64,
}

pub enum NoiseStrategy {
    PerlinInterpolation,
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new(strategy: NoiseStrategy) -> Perlin {
        let mut random_uvec: Vec<Vec3> = Vec::with_capacity(Perlin::POINT_COUNT);
        for _ in 0 .. Perlin::POINT_COUNT {
            random_uvec.push(Vec3::random(RandomVectorType::Unit));
        }

        Perlin {
            perm_x: generate_permutation(Perlin::POINT_COUNT),
            perm_y: generate_permutation(Perlin::POINT_COUNT),
            perm_z: generate_permutation(Perlin::POINT_COUNT),
            random_uvec,
            noise: match strategy {
                NoiseStrategy::PerlinInterpolation => Self::hermitian_smoothing_noise,
            },
        }
    }

    pub fn turbulence(&self, p: &Point3, depth: u32) -> f64 {
        let mut accum = 0.;
        let mut temp_p = p.clone();
        let mut weight = 1.;

        for _ in 0..depth {
            accum += weight*(self.noise)(&self, &temp_p);
            weight *= 0.5;
            temp_p = 2. * temp_p;
        }

        f64::abs(accum)
    }

    fn hermitian_smoothing_noise(&self, p: &Point3) -> f64 {
        let u = p.x - f64::floor(p.x);
        let v = p.y - f64::floor(p.y);
        let w = p.z - f64::floor(p.z);

        let i = f64::floor(p.x) as isize as usize;
        let j = f64::floor(p.y) as isize as usize;
        let k = f64::floor(p.z) as isize as usize;

        // flattened 2x2x2 array
        // let mut c = [Vec3::new(0., 0., 0.); 8];
        let mut c: Vec<&Vec3> = Vec::with_capacity(8);
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c.push(&self.random_uvec[
                        self.perm_x[(i + di) & 255]
                        ^ self.perm_y[(j + dj) & 255]
                        ^ self.perm_z[(k + dk) & 255]]);
                }
            }
        }

        perlin_interpolation(c, u, v, w)
    }
}

fn generate_permutation(n: usize) -> Vec<usize> {
    let mut v: Vec<usize> = (0..n).collect();
    v.shuffle(&mut rand::thread_rng());

    v
}

fn perlin_interpolation(c: Vec<&Vec3>, u: f64, v: f64, w: f64) -> f64 {
    let uu = u * u * (3. - 2. * u);
    let vv = v * v * (3. - 2. * v);
    let ww = w * w * (3. - 2. * w);
    let mut accum = 0.;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let idx = k + 2 * (j + 2 * i);

                let i = i as f64;
                let j = j as f64;
                let k = k as f64;

                let weight_vec = Vec3::new(u-i, v-j, w-k);
                accum +=
                    (i * uu + (1. - i) * (1. - uu))
                    * (j * vv + (1. - j) * (1. - vv))
                    * (k * ww + (1. - k) * (1. - ww))
                    * Vec3::dot(&c[idx], &weight_vec);
            }
        }
    }

    accum
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_generate_permutation() {
        let n = 256;
        let p1 = generate_permutation(n);

        // make sure every element appears once and only once. Order not important
        for i in 0..n {
            assert_eq!(p1.iter().filter(|&j| *j == i).count(), 1);
        }

        // not impossible, but it will take a few lifetimes-of-the-universe to be false(1 / 256!)
        let p2 = generate_permutation(n);
        assert_ne!(p1, p2);
    }
}
