use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    material::Material,
    ray::Ray,
    rect::{XYRect, XZRect, YZRect},
};
use cliffy::Vec3;
use std::rc::Rc;

pub struct GeoBox {
    pub min: Vec3,
    pub max: Vec3,
    pub sides: HittableList,
}

impl GeoBox {
    pub fn new(p0: &Vec3, p1: &Vec3, mat: Rc<dyn Material>) -> Self {
        let min = *p0;
        let max = *p1;

        let mut sides = HittableList::empty();
        sides.add(Rc::new(Hittable::XYRect(XYRect::new(
            mat.clone(),
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p1.z,
        ))));
        sides.add(Rc::new(Hittable::XYRect(XYRect::new(
            mat.clone(),
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p0.z,
        ))));

        sides.add(Rc::new(Hittable::XZRect(XZRect::new(
            mat.clone(),
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p1.y,
        ))));
        sides.add(Rc::new(Hittable::XZRect(XZRect::new(
            mat.clone(),
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p0.y,
        ))));

        sides.add(Rc::new(Hittable::YZRect(YZRect::new(
            mat.clone(),
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p1.x,
        ))));
        sides.add(Rc::new(Hittable::YZRect(YZRect::new(
            mat.clone(),
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p0.x,
        ))));

        Self { min, max, sides }
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }

    pub fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        Some(AABB::new(&self.min, &self.max))
    }
}
