use crate::{hittable::HitRecord, material::Material, ray::Ray, utilities};
use cliffy::Vec3;

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Vec3, Ray) {
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
