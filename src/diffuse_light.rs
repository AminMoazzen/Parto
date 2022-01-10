use crate::{color::Color, material::Material, texture::Texture};
use cliffy::{Vec2, Vec3};
use std::rc::Rc;

pub struct DiffuseLight {
    emit: Rc<Texture>,
}

impl DiffuseLight {
    pub fn new(emit: Rc<Texture>) -> Self {
        Self { emit }
    }

    pub fn with_color(c: Color) -> Self {
        let emit = Rc::new(Texture::SolidColor(c));

        Self::new(emit)
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        r_in: &crate::ray::Ray,
        rec: &crate::hittable::HitRecord,
    ) -> (bool, crate::color::Color, crate::ray::Ray) {
        (false, Default::default(), Default::default())
    }

    fn emitted(&self, uv: &Vec2, p: &Vec3) -> crate::color::Color {
        self.emit.value(uv, p)
    }
}
