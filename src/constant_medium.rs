use cliffy::{Vec3, Vector};

use crate::{
    aabb::AABB,
    color::Color,
    hittable::{HitRecord, Hittable},
    isotropic::Isotropic,
    material::Material,
    ray::Ray,
    texture::Texture,
    utilities,
};
use std::rc::Rc;

pub struct ConstantMedium {
    pub boundary: Rc<Hittable>,
    pub phase_function: Rc<dyn Material>,
    pub neg_inv_density: f32,
}

impl ConstantMedium {
    pub fn new(b: Rc<Hittable>, d: f32, a: Rc<Texture>) -> Self {
        Self {
            boundary: b,
            phase_function: Rc::new(Isotropic::new(a)),
            neg_inv_density: -1.0 / d,
        }
    }

    pub fn with_color(b: Rc<Hittable>, d: f32, c: Color) -> Self {
        Self {
            boundary: b,
            phase_function: Rc::new(Isotropic::with_color(c)),
            neg_inv_density: -1.0 / d,
        }
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // Print occasional samples when debugging. To enable, set enableDebug true.
        let enableDebug = false;
        let debugging = enableDebug && utilities::random_float() < 0.00001;

        let mut rec1;
        if let Some(r) = self.boundary.hit(r, -f32::INFINITY, f32::INFINITY) {
            rec1 = r;
        } else {
            return None;
        }

        let mut rec2;
        if let Some(r) = self.boundary.hit(r, rec1.t + 0.0001, f32::INFINITY) {
            rec2 = r;
        } else {
            return None;
        }

        if debugging {
            println!("\nt_min = {}, t_max = {}", rec1.t, rec2.t);
        }

        if rec1.t < t_min {
            rec1.t = t_min;
        }
        if rec2.t > t_max {
            rec2.t = t_max;
        }

        if rec1.t >= rec2.t {
            return None;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.direction.mag();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * utilities::random_float().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let mut rec = HitRecord::with_mat_only(self.phase_function.clone());
        rec.t = rec.t + hit_distance / ray_length;
        rec.point = r.at(rec.t);

        if debugging {
            println!(
                "hit_distance = {}\nrec.t = {}\nrec.point = {:?}",
                hit_distance, rec.t, rec.point
            );
        }

        rec.normal = Vec3::new(1.0, 0.0, 0.0); // arbitrary
        rec.front_face = true; // also arbitrary

        Some(rec)
    }

    pub fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        self.boundary.bounding_box(time0, time1)
    }
}
