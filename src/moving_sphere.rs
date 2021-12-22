use crate::{hittable::HitRecord, material::Material, ray::Ray};
use cliffy::{Vec3, Vector};
use std::rc::Rc;

pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f32,
    pub time1: f32,
    pub radius: f32,
    pub material: Rc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        center0: Vec3,
        center1: Vec3,
        time0: f32,
        time1: f32,
        radius: f32,
        material: Rc<dyn Material>,
    ) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    pub fn center(&self, time: f32) -> Vec3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center(r.time);
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
        let outward_normal = (rec.point - self.center(r.time)) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }
}
