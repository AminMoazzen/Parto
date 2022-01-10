use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    ray::Ray,
};
use cliffy::Vec3;
use std::rc::Rc;

pub struct RotateY {
    pub ptr: Rc<Hittable>,
    pub sin_theta: f32,
    pub cos_theta: f32,
    pub bbox: Option<AABB>,
}

impl RotateY {
    pub fn new(ptr: Rc<Hittable>, angle: f32) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let mut min = Vec3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut max = Vec3::new(-f32::INFINITY, -f32::INFINITY, -f32::INFINITY);

        if let Some(bbox) = ptr.bounding_box(0.0, 1.0) {
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x = i as f32 * bbox.max.x + (1 - i) as f32 * bbox.min.x;
                        let y = i as f32 * bbox.max.y + (1 - i) as f32 * bbox.min.y;
                        let z = i as f32 * bbox.max.z + (1 - i) as f32 * bbox.min.z;

                        let new_x = cos_theta * x + sin_theta * z;
                        let new_z = -sin_theta * x + cos_theta * z;

                        let tester = Vec3::new(new_x, y, new_z);

                        for c in 0..3 {
                            min[c] = min[c].min(tester[c]);
                            max[c] = max[c].max(tester[c]);
                        }
                    }
                }
            }
        }

        let bbox = Some(AABB::new(&min, &max));

        Self {
            ptr,
            sin_theta,
            cos_theta,
            bbox,
        }
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut origin = r.origin;
        let mut direction = r.direction;

        origin.x = self.cos_theta * r.origin.x - self.sin_theta * r.origin.z;
        origin.z = self.sin_theta * r.origin.x + self.cos_theta * r.origin.z;

        direction.x = self.cos_theta * r.direction.x - self.sin_theta * r.direction.z;
        direction.z = self.sin_theta * r.direction.x + self.cos_theta * r.direction.z;

        let rotated_r = Ray::with_time(origin, direction, r.time);

        if let Some(mut rec) = self.ptr.hit(&rotated_r, t_min, t_max) {
            let mut p = rec.point;
            let mut normal = rec.normal;

            p.x = self.cos_theta * rec.point.x + self.sin_theta * rec.point.z;
            p.z = -self.sin_theta * rec.point.x + self.cos_theta * rec.point.z;

            normal.x = self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z;
            normal.z = -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z;

            rec.point = p;
            rec.set_face_normal(&rotated_r, &normal);

            return Some(rec);
        } else {
            return None;
        }
    }

    pub fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        self.bbox.clone()
    }
}
