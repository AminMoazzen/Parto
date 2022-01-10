use cliffy::{Vec2, Vec3};

use crate::{
    color::Color, hittable::HitRecord, material::Material, ray::Ray, texture::Texture, utilities,
};
use std::rc::Rc;

pub struct Isotropic {
    pub albedo: Rc<Texture>,
}

impl Isotropic {
    pub fn new(albedo: Rc<Texture>) -> Self {
        Self { albedo }
    }

    pub fn with_color(c: Color) -> Self {
        Self {
            albedo: Rc::new(Texture::SolidColor(c)),
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Color, Ray) {
        let scattered = Ray::with_time(rec.point, utilities::random_in_sphere(), r_in.time);
        let attenuation = self.albedo.value(&rec.uv, &rec.point);

        (true, attenuation, scattered)
    }

    fn emitted(&self, uv: &Vec2, p: &Vec3) -> Color {
        Color::black()
    }
}
