use cliffy::{Vec3, Vector};
use rand::prelude::*;

const RAND_MAX: f32 = 1.0;

#[inline]
pub fn random() -> f32 {
    let mut rng = StdRng::from_entropy();
    rng.gen::<f32>() / (RAND_MAX + 1.0)
}

#[inline]
pub fn random_between(min: f32, max: f32) -> f32 {
    min + (max - min) * random()
}

#[inline]
pub fn random_vec3() -> Vec3 {
    Vec3::new(random(), random(), random())
}

#[inline]
pub fn random_vec3_between(min: f32, max: f32) -> Vec3 {
    Vec3::new(
        random_between(min, max),
        random_between(min, max),
        random_between(min, max),
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
