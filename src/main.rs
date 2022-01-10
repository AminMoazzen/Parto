mod aabb;
mod bvh_node;
mod camera;
mod color;
mod constant_medium;
mod dielectric;
mod diffuse_light;
mod geo_box;
mod hittable;
mod hittable_list;
mod isotropic;
mod lambertian;
mod material;
mod metal;
mod moving_sphere;
mod perlin;
mod ray;
mod rect;
mod rotate;
mod sphere;
mod texture;
mod translate;
mod utilities;

use crate::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};
use bvh_node::BVHNode;
use camera::*;
use cliffy::{Vec3, Vector};
use color::Color;
use constant_medium::ConstantMedium;
use diffuse_light::DiffuseLight;
use geo_box::GeoBox;
use hittable::Hittable;
use hittable_list::HittableList;
use image::{DynamicImage, GenericImage, Pixel};
use moving_sphere::MovingSphere;
use perlin::Perlin;
use ray::Ray;
use rect::{XYRect, XZRect, YZRect};
use rotate::RotateY;
use sphere::Sphere;
use std::{
    rc::Rc,
    time::{Instant, SystemTime},
};
use texture::Texture;
use translate::Translate;
use utilities::random_float_between;

use image::io::Reader as ImageReader;

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

fn ray_color(r: &Ray, background: &Color, world: &HittableList, depth: u32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth == 0 {
        return Color::black();
    }

    if let Some(rec) = world.hit(r, 0.001, f32::INFINITY) {
        let emitted = rec.mat.emitted(&rec.uv, &rec.point);
        let (is_scattered, attenuation, scattered) = rec.mat.scatter(r, &rec);
        if is_scattered {
            let ray_color = ray_color(&scattered, background, world, depth - 1);
            return emitted + attenuation * ray_color;
        } else {
            return emitted;
        }
    } else {
        return *background;
    }

    // let unit_direction = r.direction.normalized();
    // let t = 0.5 * (unit_direction.y + 1.0);

    // (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn write_color(
    image: &mut DynamicImage,
    x: u32,
    y: u32,
    pixel_color: &Color,
    samples_per_pixel: u32,
) {
    let mut r = pixel_color.r;
    let mut g = pixel_color.g;
    let mut b = pixel_color.b;

    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    let scale = 1.0 / samples_per_pixel as f32;
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();

    let ur = (256.0 * clamp(r, 0.0, 0.999)) as u8;
    let ug = (256.0 * clamp(g, 0.0, 0.999)) as u8;
    let ub = (256.0 * clamp(b, 0.0, 0.999)) as u8;

    let pixel = Pixel::from_channels(ur, ug, ub, 255);
    image.put_pixel(x, y, pixel);
}

fn random_scene() -> HittableList {
    let mut world = HittableList::empty();

    let checker = Rc::new(Texture::Checker(
        10.0,
        Rc::new(Texture::SolidColor(Color::new(0.2, 0.3, 0.1))),
        Rc::new(Texture::SolidColor(Color::new(0.9, 0.9, 0.9))),
    ));

    let ground_material = Rc::new(Lambertian::new(checker));
    world.add(Rc::new(Hittable::Sphere(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material.clone(),
    ))));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = utilities::random_float();
            let center = Vec3::new(
                a as f32 + 0.9 * utilities::random_float(),
                0.2,
                b as f32 + 0.9 * utilities::random_float(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).mag() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let mut albedo = utilities::random_color();
                    let random_albedo = utilities::random_color();
                    albedo *= random_albedo;
                    let center2 = center + Vec3::new(0.0, random_float_between(0.0, 0.5), 0.0);
                    world.add(Rc::new(Hittable::MovingSphere(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        Rc::new(Lambertian::with_color(albedo)),
                    ))));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = utilities::random_color_between(0.5, 1.0);
                    let fuzz = utilities::random_float_between(0.0, 0.5);
                    world.add(Rc::new(Hittable::Sphere(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Metal::new(albedo, fuzz)),
                    ))));
                } else {
                    // glass
                    world.add(Rc::new(Hittable::Sphere(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Dielectric::new(1.5)),
                    ))));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Hittable::Sphere(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    ))));

    let material2 = Rc::new(Lambertian::with_color(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Hittable::Sphere(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    ))));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Hittable::Sphere(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    ))));

    world
}

fn two_spheres() -> HittableList {
    let mut objects = HittableList::empty();

    let checker = Rc::new(Texture::Checker(
        10.0,
        Rc::new(Texture::SolidColor(Color::new(0.2, 0.3, 0.1))),
        Rc::new(Texture::SolidColor(Color::new(0.9, 0.9, 0.9))),
    ));

    objects.add(Rc::new(Hittable::Sphere(Sphere::new(
        Vec3::new(0.0, -10.0, 0.0),
        10.0,
        Rc::new(Lambertian::new(checker.clone())),
    ))));
    objects.add(Rc::new(Hittable::Sphere(Sphere::new(
        Vec3::new(0.0, 10.0, 0.0),
        10.0,
        Rc::new(Lambertian::new(checker.clone())),
    ))));

    objects
}

fn two_perlin_spheres() -> HittableList {
    let mut objects = HittableList::empty();

    let per_text = Rc::new(Texture::Noise(4.0, Perlin::new()));
    objects.add(Rc::new(Hittable::Sphere(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new(per_text.clone())),
    ))));
    objects.add(Rc::new(Hittable::Sphere(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Rc::new(Lambertian::new(per_text.clone())),
    ))));

    objects
}

fn earth() -> HittableList {
    let earth_texture = Rc::new(Texture::Image(
        ImageReader::open("res/earthmap.jpg")
            .unwrap()
            .decode()
            .unwrap(),
    ));
    let earth_surface = Rc::new(Lambertian::new(earth_texture.clone()));
    let globe = Rc::new(Hittable::Sphere(Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        2.0,
        earth_surface.clone(),
    )));

    HittableList::new(globe)
}

fn simple_light() -> HittableList {
    let mut objects = HittableList::empty();

    let pertext = Rc::new(Texture::Noise(4.0, Perlin::new()));
    objects.add(Rc::new(Hittable::Sphere(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new(pertext.clone())),
    ))));
    objects.add(Rc::new(Hittable::Sphere(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Rc::new(Lambertian::new(pertext.clone())),
    ))));

    let diff_light = Rc::new(DiffuseLight::with_color(Color::new(4.0, 4.0, 4.0)));
    objects.add(Rc::new(Hittable::XYRect(XYRect::new(
        diff_light.clone(),
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
    ))));

    objects
}

fn cornell_box() -> HittableList {
    let mut objects = HittableList::empty();

    let red = Rc::new(Lambertian::new(Rc::new(Texture::SolidColor(Color::new(
        0.65, 0.05, 0.05,
    )))));
    let white = Rc::new(Lambertian::new(Rc::new(Texture::SolidColor(Color::new(
        0.73, 0.73, 0.73,
    )))));
    let green = Rc::new(Lambertian::new(Rc::new(Texture::SolidColor(Color::new(
        0.12, 0.45, 0.15,
    )))));
    let light = Rc::new(DiffuseLight::new(Rc::new(Texture::SolidColor(Color::new(
        15.0, 15.0, 15.0,
    )))));

    objects.add(Rc::new(Hittable::YZRect(YZRect::new(
        green.clone(),
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
    ))));
    objects.add(Rc::new(Hittable::YZRect(YZRect::new(
        red.clone(),
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
    ))));
    objects.add(Rc::new(Hittable::XZRect(XZRect::new(
        light.clone(),
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
    ))));
    objects.add(Rc::new(Hittable::XZRect(XZRect::new(
        white.clone(),
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
    ))));
    objects.add(Rc::new(Hittable::XZRect(XZRect::new(
        white.clone(),
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
    ))));
    objects.add(Rc::new(Hittable::XYRect(XYRect::new(
        white.clone(),
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
    ))));

    let mut box1 = Rc::new(Hittable::Box(GeoBox::new(
        &Vec3::new(0.0, 0.0, 0.0),
        &Vec3::new(165.0, 330.0, 165.0),
        white.clone(),
    )));
    box1 = Rc::new(Hittable::RotateY(RotateY::new(box1, 15.0)));
    box1 = Rc::new(Hittable::Translate(Translate::new(
        box1,
        Vec3::new(265.0, 0.0, 295.0),
    )));
    objects.add(box1);

    let mut box2 = Rc::new(Hittable::Box(GeoBox::new(
        &Vec3::new(0.0, 0.0, 0.0),
        &Vec3::new(165.0, 165.0, 165.0),
        white.clone(),
    )));
    box2 = Rc::new(Hittable::RotateY(RotateY::new(box2, -18.0)));
    box2 = Rc::new(Hittable::Translate(Translate::new(
        box2,
        Vec3::new(130.0, 0.0, 65.0),
    )));
    objects.add(box2);

    objects
}

fn cornell_smoke() -> HittableList {
    let mut objects = HittableList::empty();

    let red = Rc::new(Lambertian::new(Rc::new(Texture::SolidColor(Color::new(
        0.65, 0.05, 0.05,
    )))));
    let white = Rc::new(Lambertian::new(Rc::new(Texture::SolidColor(Color::new(
        0.73, 0.73, 0.73,
    )))));
    let green = Rc::new(Lambertian::new(Rc::new(Texture::SolidColor(Color::new(
        0.12, 0.45, 0.15,
    )))));
    let light = Rc::new(DiffuseLight::new(Rc::new(Texture::SolidColor(Color::new(
        15.0, 15.0, 15.0,
    )))));

    objects.add(Rc::new(Hittable::YZRect(YZRect::new(
        green.clone(),
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
    ))));
    objects.add(Rc::new(Hittable::YZRect(YZRect::new(
        red.clone(),
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
    ))));
    objects.add(Rc::new(Hittable::XZRect(XZRect::new(
        light.clone(),
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
    ))));
    objects.add(Rc::new(Hittable::XZRect(XZRect::new(
        white.clone(),
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
    ))));
    objects.add(Rc::new(Hittable::XZRect(XZRect::new(
        white.clone(),
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
    ))));
    objects.add(Rc::new(Hittable::XYRect(XYRect::new(
        white.clone(),
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
    ))));

    let mut box1 = Rc::new(Hittable::Box(GeoBox::new(
        &Vec3::new(0.0, 0.0, 0.0),
        &Vec3::new(165.0, 330.0, 165.0),
        white.clone(),
    )));
    box1 = Rc::new(Hittable::RotateY(RotateY::new(box1, 15.0)));
    box1 = Rc::new(Hittable::Translate(Translate::new(
        box1,
        Vec3::new(265.0, 0.0, 295.0),
    )));

    let mut box2 = Rc::new(Hittable::Box(GeoBox::new(
        &Vec3::new(0.0, 0.0, 0.0),
        &Vec3::new(165.0, 165.0, 165.0),
        white.clone(),
    )));
    box2 = Rc::new(Hittable::RotateY(RotateY::new(box2, -18.0)));
    box2 = Rc::new(Hittable::Translate(Translate::new(
        box2,
        Vec3::new(130.0, 0.0, 65.0),
    )));

    objects.add(Rc::new(Hittable::ConstantMedium(
        ConstantMedium::with_color(box1, 0.01, Color::black()),
    )));
    objects.add(Rc::new(Hittable::ConstantMedium(
        ConstantMedium::with_color(box2, 0.01, Color::white()),
    )));

    objects
}

fn final_scene() -> HittableList {
    let mut boxes1 = HittableList::empty();
    let ground = Rc::new(Lambertian::with_color(Color::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f32 * w;
            let z0 = -1000.0 + j as f32 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = utilities::random_float_between(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Rc::new(Hittable::Box(GeoBox::new(
                &Vec3::new(x0, y0, z0),
                &Vec3::new(x1, y1, z1),
                ground.clone(),
            ))));
        }
    }

    let mut objects = HittableList::empty();

    objects.add(Rc::new(Hittable::Node(BVHNode::new(&mut boxes1, 0.0, 1.0))));

    let light = Rc::new(DiffuseLight::with_color(Color::white() * 7.0));
    objects.add(Rc::new(Hittable::XZRect(XZRect::new(
        light.clone(),
        123.0,
        423.0,
        147.0,
        412.0,
        554.0,
    ))));

    let center1 = Vec3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_mat = Rc::new(Lambertian::with_color(Color::new(0.7, 0.3, 0.1)));
    objects.add(Rc::new(Hittable::MovingSphere(MovingSphere::new(
        center1,
        center2,
        0.0,
        1.0,
        50.0,
        moving_sphere_mat.clone(),
    ))));

    objects.add(Rc::new(Hittable::Sphere(Sphere::new(
        Vec3::new(260.0, 150.0, 45.0),
        50.0,
        Rc::new(Dielectric::new(1.5)),
    ))));
    objects.add(Rc::new(Hittable::Sphere(Sphere::new(
        Vec3::new(0.0, 150.0, 145.0),
        50.0,
        Rc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    ))));

    let boundary = Rc::new(Hittable::Sphere(Sphere::new(
        Vec3::new(360.0, 150.0, 145.0),
        70.0,
        Rc::new(Dielectric::new(1.5)),
    )));
    objects.add(boundary.clone());
    objects.add(Rc::new(Hittable::ConstantMedium(
        ConstantMedium::with_color(boundary.clone(), 0.2, Color::new(0.2, 0.4, 0.9)),
    )));

    let boundary = Rc::new(Hittable::Sphere(Sphere::new(
        Vec3::zero(),
        5000.0,
        Rc::new(Dielectric::new(1.5)),
    )));
    objects.add(Rc::new(Hittable::ConstantMedium(
        ConstantMedium::with_color(boundary, 0.0001, Color::white()),
    )));

    let emat = Rc::new(Lambertian::new(Rc::new(Texture::Image(
        ImageReader::open("res/earthmap.jpg")
            .unwrap()
            .decode()
            .unwrap(),
    ))));
    objects.add(Rc::new(Hittable::Sphere(Sphere::new(
        Vec3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    ))));
    let pertext = Rc::new(Texture::Noise(0.1, Perlin::new()));
    objects.add(Rc::new(Hittable::Sphere(Sphere::new(
        Vec3::new(220.0, 280.0, 300.0),
        80.0,
        Rc::new(Lambertian::new(pertext)),
    ))));

    let mut boxes2 = HittableList::empty();
    let white = Rc::new(Lambertian::with_color(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Rc::new(Hittable::Sphere(Sphere::new(
            utilities::random_vec3_between(0.0, 165.0),
            10.0,
            white.clone(),
        ))));
    }

    let node = Hittable::Node(BVHNode::new(&mut boxes2, 0.0, 1.0));
    let rotate = Hittable::RotateY(RotateY::new(Rc::new(node), 15.0));
    let translate = Hittable::Translate(Translate::new(
        Rc::new(rotate),
        Vec3::new(-100.0, 270.0, 395.0),
    ));

    objects.add(Rc::new(translate));

    objects
}

fn main() {
    // Image
    let mut aspect_ratio = 16.0 / 9.0;
    let mut image_width = 400;
    let mut samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let world;

    let look_from;
    let look_at;
    let mut vfov = 40.0;
    let mut aperture = 0.0;
    let background;

    let scene = 0;

    match scene {
        1 => {
            world = random_scene();
            background = Color::new(0.7, 0.8, 1.0);
            look_from = Vec3::new(13.0, 2.0, 3.0);
            look_at = Vec3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.1;
        }

        2 => {
            world = two_spheres();
            background = Color::new(0.7, 0.8, 1.0);
            look_from = Vec3::new(13.0, 2.0, 3.0);
            look_at = Vec3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }

        3 => {
            world = two_perlin_spheres();
            background = Color::new(0.7, 0.8, 1.0);
            look_from = Vec3::new(13.0, 2.0, 3.0);
            look_at = Vec3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }

        4 => {
            world = earth();
            background = Color::new(0.7, 0.8, 1.0);
            look_from = Vec3::new(13.0, 2.0, 3.0);
            look_at = Vec3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }

        5 => {
            world = simple_light();
            samples_per_pixel = 400;
            background = Color::black();
            look_from = Vec3::new(26.0, 3.0, 6.0);
            look_at = Vec3::new(0.0, 2.0, 0.0);
            vfov = 20.0;
        }

        6 => {
            world = cornell_box();
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 200;
            background = Color::black();
            look_from = Vec3::new(278.0, 278.0, -800.0);
            look_at = Vec3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }

        7 => {
            world = cornell_smoke();
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 200;
            background = Color::black();
            look_from = Vec3::new(278.0, 278.0, -800.0);
            look_at = Vec3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }

        _ => {
            world = final_scene();
            aspect_ratio = 1.0;
            image_width = 800;
            samples_per_pixel = 10000;
            background = Color::black();
            look_from = Vec3::new(478.0, 278.0, -600.0);
            look_at = Vec3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
    }

    // Camera
    let vup = Vec3::up();
    let dist_to_focus = 10.0;
    let mut image_height = (image_width as f32 / aspect_ratio) as u32;
    let mut image = DynamicImage::new_rgb8(image_width, image_height);

    let cam = Camera::with_time(
        look_from,
        look_at,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    let now = Instant::now();

    // Render
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Color::black();
            for _s in 0..samples_per_pixel {
                let u = (i as f32 + utilities::random_float()) / (image_width - 1) as f32;
                let v = (j as f32 + utilities::random_float()) / (image_height - 1) as f32;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &background, &world, max_depth);
            }

            write_color(
                &mut image,
                i,
                image_height - 1 - j,
                &pixel_color,
                samples_per_pixel,
            );
        }
    }

    let elapsed = now.elapsed().as_secs();
    let hours = elapsed / 3600;
    let minutes = (elapsed / 60) % 60;
    let seconds = elapsed % 60;
    println!("Time elapsed: {}:{}:{}", hours, minutes, seconds);

    let image_path = "./output.png";

    image.save(&image_path).unwrap();
}
