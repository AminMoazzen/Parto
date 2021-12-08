mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;

use camera::*;
use cliffy::{Vec3, Vector};
use hittable::Hittable;
use hittable_list::HittableList;
use image::{DynamicImage, GenericImage, Pixel};
use rand::prelude::*;
use ray::Ray;
use sphere::Sphere;

const RAND_MAX: f32 = 1.0;

fn random() -> f32 {
    let mut rng = StdRng::from_entropy();
    rng.gen::<f32>() / (RAND_MAX + 1.0)
}

fn random_between(min: f32, max: f32) -> f32 {
    min + (max - min) * random()
}

#[inline]
fn clamp(x: f32, min: f32, max: f32) -> f32 {
    let mut result = x;

    if x < min {
        result = min;
    } else if x > max {
        result = max;
    }

    result
}

fn ray_color(r: &Ray, world: &dyn Hittable) -> Vec3 {
    let (hit, rec) = world.hit(r, 0.0, f32::INFINITY);
    if hit {
        let hit_rec = rec.unwrap();
        return 0.5 * (hit_rec.normal + Vec3::new(1.0, 1.0, 1.0));
    }

    let unit_direction = r.direction.normalized();
    let t = 0.5 * (unit_direction.y + 1.0);

    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn write_color(
    image: &mut DynamicImage,
    x: u32,
    y: u32,
    pixel_color: &Vec3,
    samples_per_pixel: u32,
) {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    let scale = 1.0 / samples_per_pixel as f32;
    r *= scale;
    g *= scale;
    b *= scale;

    let ur = (256.0 * clamp(r, 0.0, 0.999)) as u8;
    let ug = (256.0 * clamp(g, 0.0, 0.999)) as u8;
    let ub = (256.0 * clamp(b, 0.0, 0.999)) as u8;

    let pixel = Pixel::from_channels(ur, ug, ub, 255);
    image.put_pixel(x, y, pixel);
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as u32;
    let samples_per_pixel = 100;

    let mut image = DynamicImage::new_rgb8(image_width, image_height);

    // World
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let cam = Camera::new();

    // Render
    let mut progress = 0;
    let total_pixels = image_width * image_height;
    print!("{} / {}", progress, total_pixels);

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel {
                let u = (i as f32 + random()) / (image_width - 1) as f32;
                let v = (j as f32 + random()) / (image_height - 1) as f32;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }

            write_color(
                &mut image,
                i,
                image_height - 1 - j,
                &pixel_color,
                samples_per_pixel,
            );

            progress += 1;
        }
        print!("\r {} / {}", progress, total_pixels);
    }
    println!("");

    let image_path = "./output.png";

    image.save(&image_path).unwrap();
}
