use crate::{aabb::AABB, hittable::HitRecord, material::Material, Ray};
use cliffy::{Vec2, Vec3, Vector};
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

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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
        rec.uv = Self::get_uv(&outward_normal);

        Some(rec)
    }

    pub fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        let offset_by_radius = Vec3::new(self.radius, self.radius, self.radius);
        Some(AABB {
            min: self.center - offset_by_radius,
            max: self.center + offset_by_radius,
        })
    }

    fn get_uv(p: &Vec3) -> Vec2 {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

        use std::f32::consts::PI;

        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + PI;

        Vec2::new(phi / (2.0 * PI), theta / PI)
    }
}
