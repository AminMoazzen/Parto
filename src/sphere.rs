use crate::{hittable::*, material::Material, Ray};
use cliffy::*;
use std::rc::Rc;

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.mag_sq();
        let half_b = oc.dot(r.direction);
        let c = oc.mag_sq() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = f32::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;

            if root < t_min || t_max < root {
                return None;
            }
        }

        let mut rec = HitRecord::with_mat_only(self.material.clone());

        rec.t = root;
        rec.point = r.at(rec.t);
        let outward_normal = (rec.point - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }
}
