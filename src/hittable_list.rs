use crate::{hittable::*, ray::Ray};

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Hittable>,
}

impl HittableList {
    pub fn new(object: Hittable) -> Self {
        Self {
            objects: vec![object],
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Hittable) {
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
}
