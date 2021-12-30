use crate::{color::Color, hittable::HitRecord, material::Material, ray::Ray, utilities};
use cliffy::Vector;

pub struct Dielectric {
    pub ir: f32, // Index of Refraction
}

impl Dielectric {
    pub fn new(ir: f32) -> Self {
        Self { ir }
    }

    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Color, Ray) {
        let attenuation = Color::white();
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction.normalized();
        let cos_theta = rec.normal.dot(-unit_direction).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction;

        if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > utilities::random() {
            direction = unit_direction.reflected_normal(rec.normal);
        } else {
            direction = utilities::refract(&unit_direction, &rec.normal, refraction_ratio);
        }

        let scattered = Ray::with_time(rec.point, direction, r_in.time);

        (true, attenuation, scattered)
    }
}
