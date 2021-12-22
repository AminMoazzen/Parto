use std::rc::Rc;

use crate::{material::Material, moving_sphere::MovingSphere, sphere::Sphere, Ray};
use cliffy::*;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub mat: Rc<dyn Material>,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(point: Vec3, normal: Vec3, mat: Rc<dyn Material>, t: f32, front_face: bool) -> Self {
        Self {
            point,
            normal,
            mat,
            t,
            front_face,
        }
    }

    pub fn with_mat_only(mat: Rc<dyn Material>) -> Self {
        Self {
            point: Vec3::zero(),
            normal: Vec3::up(),
            mat,
            t: 0.0,
            front_face: false,
        }
    }
    #[inline]
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction.dot(*outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub enum Hittable {
    Sphere(Sphere),
    MovingSphere(MovingSphere),
}

impl Hittable {
    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match &self {
            Self::Sphere(s) => s.hit(r, t_min, t_max),
            Hittable::MovingSphere(ms) => ms.hit(r, t_min, t_max),
        }
    }
}
