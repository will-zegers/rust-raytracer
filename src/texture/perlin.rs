use rand::Rng;
use rand::seq::SliceRandom;

use crate::geometry::Point3;

pub struct Perlin {
    random_float: [f64; Perlin::POINT_COUNT],
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Perlin {
        let mut rng = rand::thread_rng();

        let mut random_float = [0_f64; Perlin::POINT_COUNT];
        for elem in random_float.iter_mut() {
            *elem = rng.gen();
        }

        Perlin {
            random_float,
            perm_x: generate_permutation(Perlin::POINT_COUNT),
            perm_y: generate_permutation(Perlin::POINT_COUNT),
            perm_z: generate_permutation(Perlin::POINT_COUNT),
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let i = ((4. * f64::abs(p.x)) as usize) & (Perlin::POINT_COUNT - 1);
        let j = ((4. * f64::abs(p.y)) as usize) & (Perlin::POINT_COUNT - 1);
        let k = ((4. * f64::abs(p.z)) as usize) & (Perlin::POINT_COUNT - 1);

        let index = self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k];
        self.random_float[index]
    }
}

fn generate_permutation(n: usize) -> Vec<usize> {
    let mut v: Vec<usize> = (0..n).collect();
    v.shuffle(&mut rand::thread_rng());

    v
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
