use crate::{hittable::HitRecord, ray::Ray};
use cliffy::Vec3;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Vec3, Ray);
}
