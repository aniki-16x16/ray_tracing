pub mod color;
pub mod geometry;
pub mod hittable;
pub mod math;
pub mod ray;
pub mod vec;

use std::{
    fs::File,
    io::{Write, stdout},
};

use rand::Rng;

use crate::{
    color::write_color,
    geometry::Sphere,
    hittable::{Hittable, HittableList},
    ray::Ray,
    vec::{Point3, Vec3},
};

const SAMPLES_PER_PIXEL: i32 = 10;

fn main() -> std::io::Result<()> {
    let mut buffer = File::create("test.ppm")?;
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio).floor() as i32;
    let image_height = if image_height < 1 { 1 } else { image_height };

    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let viewport_uv = Vec3::new(viewport_width, -viewport_height, 0.0);
    let pixel_delta_uv = Vec3::new(
        viewport_width / image_width as f64,
        -viewport_height / image_height as f64,
        0.0,
    );

    let camera_center = Point3::zero();
    let focal_length = 1.0;
    let first_pixel = camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_uv / 2.0
        + pixel_delta_uv / 2.0;

    let mut scene_objects = HittableList::new();
    scene_objects.push(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    scene_objects.push(Sphere::new(Vec3::new(0.0, -100.6, -1.0), 100.0));

    let mut rng = rand::rng();
    buffer.write(format!("P3\n{image_width} {image_height}\n255\n").as_bytes())?;
    for row in 0..image_height {
        print!("\r{:4} / {:4}", row + 1, image_height);
        stdout().flush().unwrap();
        for col in 0..image_width {
            let mut color = Vec3::zero();
            for _ in 0..SAMPLES_PER_PIXEL {
                let offset = Vec3::new(
                    col as f64 + rng.random::<f64>() - 0.5,
                    row as f64 + rng.random::<f64>() - 0.5,
                    0.0,
                );
                let current = first_pixel + offset * pixel_delta_uv;
                let ray = Ray::new(camera_center, (current - camera_center).normalize());
                let factor = ray.direction().y() * 0.5 + 0.5;
                color = color
                    + if let Some(result) = scene_objects.hit(&ray, (0.0, 100.0)) {
                        *result.normal() * 0.5 + 0.5
                    } else {
                        Vec3::mix(Vec3::one(), Vec3::new(0.5, 0.7, 1.0), factor)
                    }
            }
            color = color / SAMPLES_PER_PIXEL as f64;
            color = color.sqrt();
            buffer.write(write_color(&color).as_bytes())?;
        }
    }
    println!("\ndone");
    Ok(())
}
