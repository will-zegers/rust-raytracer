pub mod base;
pub mod dielectric;
pub mod lambertian;
pub mod metal;

use crate::vec3;
use crate::vec3::Vec3;

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = f64::min(vec3::dot(&(-uv), &n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())) * n;

    r_out_perp + r_out_parallel
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - 2.0 * vec3::dot(v, n) * n
}

#[allow(dead_code)]
fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let r0 = (1. - ref_idx) * (1. - ref_idx) / ((1. + ref_idx) * (1. + ref_idx));
    r0 + (1. - r0) * f64::powi(1. - cosine, 5)
}
