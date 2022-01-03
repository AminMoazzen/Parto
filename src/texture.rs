use crate::{
    color::Color,
    perlin::{self, Perlin},
};
use cliffy::{Vec2, Vec3};
use std::rc::Rc;

pub enum Texture {
    SolidColor(Color),
    Checker(f32, Rc<Texture>, Rc<Texture>),
    Noise(f32, Perlin),
}

impl Texture {
    pub fn value(&self, uv: Vec2, p: &Vec3) -> Color {
        match self {
            Self::SolidColor(c) => *c,
            Self::Checker(size, even, odd) => {
                let sines = (size * p.x).sin() * (size * p.y).sin() * (size * p.z).sin();
                if sines < 0.0 {
                    odd.value(uv, p)
                } else {
                    even.value(uv, p)
                }
            }
            Self::Noise(scale, perlin) => {
                // Color::white() * 0.5 * (1.0 + perlin.noise(&(*scale * *p)))
                Color::white() * 0.5 * (1.0 + (scale * p.z + 10.0 * perlin.turb(p, 7)).sin())
            }
        }
    }
}
