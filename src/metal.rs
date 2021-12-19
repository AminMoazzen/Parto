use crate::{hittable::HitRecord, material::Material, ray::Ray, utilities};
use cliffy::{Vec3, Vector};

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Vec3, Ray) {
        let relflected = r_in.direction.normalized().reflected(rec.normal);
        let scattered = Ray::new(
            rec.point,
            relflected + self.fuzz * utilities::random_in_sphere(),
        );
        let attenuation = self.albedo;

        (
            scattered.direction.dot(rec.normal) > 0.0,
            attenuation,
            scattered,
        )
    }
}
