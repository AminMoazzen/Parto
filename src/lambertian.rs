use crate::{
    color::Color, hittable::HitRecord, material::Material, ray::Ray, texture::Texture, utilities,
};
use std::rc::Rc;

pub struct Lambertian {
    pub albedo: Rc<Texture>,
}

impl Lambertian {
    pub fn new(albedo: Rc<Texture>) -> Self {
        Self { albedo }
    }

    pub fn with_color(c: Color) -> Self {
        Self {
            albedo: Rc::new(Texture::SolidColor(c)),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Color, Ray) {
        let mut scatter_direction = rec.normal + utilities::random_unit_vec3();

        // Catch degenerate scatter direction
        if utilities::near_zero(&scatter_direction) {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::with_time(rec.point, scatter_direction, r_in.time);
        let attenuation = self.albedo.value(rec.uv, &rec.point);

        (true, attenuation, scattered)
    }
}
