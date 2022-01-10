use crate::{aabb::AABB, hittable::*, ray::Ray};
use std::rc::Rc;

pub struct HittableList {
    pub objects: Vec<Rc<Hittable>>,
}

impl HittableList {
    pub fn new(object: Rc<Hittable>) -> Self {
        Self {
            objects: vec![object],
        }
    }

    pub fn empty() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<Hittable>) {
        self.objects.push(object);
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(hit_record) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = hit_record.t;
                temp_rec = Some(hit_record);
            }
        }

        temp_rec
    }

    pub fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }

        let mut output_box = Default::default();
        let mut first_box = true;
        for object in &self.objects {
            if let Some(temp_box) = object.bounding_box(time0, time1) {
                output_box = if first_box {
                    temp_box
                } else {
                    AABB::surrounding_box(output_box, temp_box)
                };
                first_box = false;
            } else {
                return None;
            }
        }

        Some(output_box)
    }
}
