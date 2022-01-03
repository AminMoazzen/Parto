use crate::{
    color::Color,
    perlin::{self, Perlin},
};
use cliffy::{Vec2, Vec3};
use image::{DynamicImage, GenericImageView};
use std::rc::Rc;

pub enum Texture {
    SolidColor(Color),
    Checker(f32, Rc<Texture>, Rc<Texture>),
    Noise(f32, Perlin),
    Image(DynamicImage),
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
            Self::Image(img) => {
                // Clamp input texture coordinates to [0,1] x [1,0]
                let u = uv.x.clamp(0.0, 1.0);
                let v = 1.0 - uv.y.clamp(0.0, 1.0); // Flip V to image coordinates

                // println!("({}, {})", uv.x, uv.y);
                let width = img.width();
                let height = img.height();

                let mut i = (u * width as f32) as u32;
                let mut j = (v * height as f32) as u32;

                // Clamp integer mapping, since actual coordinates should be less than 1.0
                if i >= width {
                    i = width - 1;
                }
                if j >= height {
                    j = height - 1;
                }

                let color_scale = 1.0 / 255.0;
                let pixel = img.get_pixel(i, j);

                let r = pixel[0] as f32 * color_scale;
                let g = pixel[1] as f32 * color_scale;
                let b = pixel[2] as f32 * color_scale;

                Color::new(r, g, b)
            }
        }
    }
}
