use cliffy::{Vec3, Vector};
use rand::prelude::*;

use crate::color::Color;

#[inline]
pub fn random_float() -> f32 {
    let mut rng = StdRng::from_entropy();
    rng.gen_range(0.0..1.0)
}

#[inline]
pub fn random_float_between(min: f32, max: f32) -> f32 {
    min + (max - min) * random_float()
}

#[inline]
pub fn random_int(min: i32, max: i32) -> i32 {
    rand::thread_rng().gen_range(min..=max)
}

#[inline]
pub fn random_vec3() -> Vec3 {
    Vec3::new(random_float(), random_float(), random_float())
}

#[inline]
pub fn random_vec3_between(min: f32, max: f32) -> Vec3 {
    Vec3::new(
        random_float_between(min, max),
        random_float_between(min, max),
        random_float_between(min, max),
    )
}

#[inline]
pub fn random_color() -> Color {
    Color::new(random_float(), random_float(), random_float())
}

#[inline]
pub fn random_color_between(min: f32, max: f32) -> Color {
    Color::new(
        random_float_between(min, max),
        random_float_between(min, max),
        random_float_between(min, max),
    )
}

#[inline]
pub fn random_in_sphere() -> Vec3 {
    loop {
        let p = random_vec3_between(-1.0, 1.0);
        if p.mag_sq() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vec3() -> Vec3 {
    random_in_sphere().normalized()
}

pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_sphere();

    // If in the same hemisphere as the normal
    if in_unit_sphere.dot(*normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

#[inline]
pub fn near_zero(vector: &Vec3) -> bool {
    let s = 1e-8;
    vector.x < s && vector.y < s && vector.z < s
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = n.dot(-*uv).min(1.0);
    let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
    let r_out_parallel = -(1.0 - r_out_perp.mag_sq()).abs().sqrt() * *n;

    r_out_perp + r_out_parallel
}

pub fn random_in_disk() -> Vec3 {
    loop {
        let p = Vec3::new(
            random_float_between(-1.0, 1.0),
            random_float_between(-1.0, 1.0),
            0.0,
        );
        if p.mag_sq() < 1.0 {
            return p;
        }
    }
}
