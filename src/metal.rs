use crate::{color::Color, hittable::HitRecord, material::Material, ray::Ray, utilities};
use cliffy::Vector;

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Color, Ray) {
        let relflected = r_in.direction.normalized().reflected(rec.normal);
        let scattered = Ray::with_time(
            rec.point,
            relflected + self.fuzz * utilities::random_in_sphere(),
            r_in.time,
        );
        let attenuation = self.albedo;

        (
            scattered.direction.dot(rec.normal) > 0.0,
            attenuation,
            scattered,
        )
    }
}
