use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    ray::Ray,
};
use rand::prelude::*;
use std::{cmp::Ordering, rc::Rc};

pub struct BVHNode {
    pub left: Rc<Hittable>,
    pub right: Rc<Hittable>,
    pub bbox: AABB,
}

impl BVHNode {
    pub fn new(list: &mut HittableList, time0: f32, time1: f32) -> Self {
        let end = list.objects.len();
        Self::new2(&mut list.objects, 0, end, time0, time1)
    }

    pub fn new2(
        src_objects: &mut [Rc<Hittable>],
        start: usize,
        end: usize,
        time0: f32,
        time1: f32,
    ) -> Self {
        let mut objects = src_objects;

        let axis = rand::thread_rng().gen_range(0..=2);
        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            _ => box_z_compare,
        };

        let object_span = end - start;

        let left;
        let right;
        match object_span {
            1 => {
                left = objects[start].clone();
                right = objects[start].clone();
            }
            2 => {
                if comparator(&objects[start], &objects[start + 1]) == Ordering::Less {
                    left = objects[start].clone();
                    right = objects[start + 1].clone();
                } else {
                    left = objects[start + 1].clone();
                    right = objects[start].clone();
                }
            }
            _ => {
                objects.sort_by(comparator);

                let mid = start + object_span / 2;
                left = Rc::new(Hittable::Node(BVHNode::new2(
                    objects, start, mid, time0, time1,
                )));
                right = Rc::new(Hittable::Node(BVHNode::new2(
                    objects, mid, end, time0, time1,
                )));
            }
        };

        let mut bbox = Default::default();

        if let Some(box_left) = left.bounding_box(time0, time1) {
            if let Some(box_right) = right.bounding_box(time0, time1) {
                bbox = AABB::surrounding_box(box_left, box_right);
            } else {
                println!("No bounding box in bvh_node constructor.");
            }
        } else {
            println!("No bounding box in bvh_node constructor.");
        };

        Self { left, right, bbox }
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if !self.bbox.hit(r, t_min, t_max) {
            return None;
        }

        if let Some(l_rec) = self.left.hit(r, t_min, t_max) {
            if let Some(r_rec) = self.right.hit(r, t_min, l_rec.t) {
                return Some(r_rec);
            } else {
                return Some(l_rec);
            }
        } else {
            if let Some(r_rec) = self.right.hit(r, t_min, t_max) {
                return Some(r_rec);
            } else {
                return None;
            }
        }
    }

    pub fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        Some(self.bbox.clone())
    }
}

#[inline]
fn box_compare(a: &Rc<Hittable>, b: &Rc<Hittable>, axis: usize) -> Ordering {
    if let Some(box_a) = a.bounding_box(0.0, 0.0) {
        if let Some(box_b) = b.bounding_box(0.0, 0.0) {
            return box_a.min[axis].partial_cmp(&box_b.min[axis]).unwrap();
        } else {
            println!("No bounding box in bvh_node constructor.");
        }
    } else {
        println!("No bounding box in bvh_node constructor.");
    };

    Ordering::Equal
}

fn box_x_compare(a: &Rc<Hittable>, b: &Rc<Hittable>) -> Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &Rc<Hittable>, b: &Rc<Hittable>) -> Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &Rc<Hittable>, b: &Rc<Hittable>) -> Ordering {
    box_compare(a, b, 2)
}
