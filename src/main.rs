mod hittable;
mod hittable_list;
mod ray;
mod sphere;

use cliffy::{Vec3, Vector};
use hittable::Hittable;
use hittable_list::HittableList;
use image::{DynamicImage, GenericImage, Pixel};
use ray::Ray;
use sphere::Sphere;

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

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as u32;

    let mut image = DynamicImage::new_rgb8(image_width, image_height);

    // World
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_lenght = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_lenght);

    // Render
    let mut progress = 0;
    let total_pixels = image_width * image_height;
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = i as f32 / (image_width - 1) as f32;
            let v = j as f32 / (image_height - 1) as f32;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&r, &world);
            write_color(&mut image, i, image_height - 1 - j, &pixel_color);

            progress += 1;
        }
        print!("\r {} / {}", progress, total_pixels);
    }
    println!("");

    let image_path = "./output.png";

    image.save(&image_path).unwrap();
}

fn write_color(image: &mut DynamicImage, x: u32, y: u32, pixel_color: &Vec3) {
    let ur = (255.999 * pixel_color.x) as u8;
    let ug = (255.999 * pixel_color.y) as u8;
    let ub = (255.999 * pixel_color.z) as u8;

    let pixel = Pixel::from_channels(ur, ug, ub, 255);
    image.put_pixel(x, y, pixel);
}
