pub mod color;
pub mod math;
pub mod ray;
pub mod vec;

use std::{fs::File, io::Write};

use crate::{
    color::write_color,
    ray::Ray,
    vec::{Point3, Vec3},
};

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
    let first_pixel = camera_center
        .subtract(&Vec3::new(0.0, 0.0, focal_length))
        .subtract(&viewport_uv.divide(2.0))
        .add(&pixel_delta_uv.divide(2.0));

    buffer.write(format!("P3\n{image_width} {image_height}\n255\n").as_bytes())?;
    for row in 0..image_height {
        for col in 0..image_width {
            let offset = Vec3::new(
                pixel_delta_uv.x() * col as f64,
                pixel_delta_uv.y() * row as f64,
                0.0,
            );
            let current = first_pixel.add(&offset);
            let ray = Ray::new(
                camera_center.clone(),
                current.subtract(&camera_center).normalize(),
            );
            let factor = ray.direction().y() * 0.5 + 0.5;
            let color = Vec3::mix(&Vec3::one(), &Vec3::new(0.5, 0.7, 1.0), factor);
            buffer.write(write_color(&color).as_bytes())?;
        }
    }
    Ok(())
}
