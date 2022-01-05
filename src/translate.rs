use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    ray::Ray,
};
use cliffy::Vec3;
use std::rc::Rc;

pub struct Translate {
    ptr: Rc<Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(ptr: Rc<Hittable>, offset: Vec3) -> Self {
        Self { ptr, offset }
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let moved_r = Ray::with_time(r.origin - self.offset, r.direction, r.time);

        if let Some(mut rec) = self.ptr.hit(&moved_r, t_min, t_max) {
            rec.point += self.offset;
            let rec_normal = rec.normal;
            rec.set_face_normal(&moved_r, &rec_normal);

            return Some(rec);
        } else {
            return None;
        }
    }

    pub fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        if let Some(bbox) = self.ptr.bounding_box(time0, time1) {
            return Some(AABB::new(
                &(bbox.min + self.offset),
                &(bbox.max + self.offset),
            ));
        } else {
            return None;
        }
    }
}
