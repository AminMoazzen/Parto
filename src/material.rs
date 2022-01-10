use crate::{color::Color, hittable::HitRecord, ray::Ray};
use cliffy::{Vec2, Vec3};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Color, Ray);

    fn emitted(&self, uv: &Vec2, p: &Vec3) -> Color;
}
