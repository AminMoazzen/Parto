use crate::{color::Color, hittable::HitRecord, material::Material, ray::Ray, utilities};

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
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
        let attenuation = self.albedo;

        (true, attenuation, scattered)
    }
}
