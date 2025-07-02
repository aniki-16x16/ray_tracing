use std::io::Write;
use std::{fs::File, io::stdout};

use crate::color::{Color, write_color};
use crate::hittable::Hittable;
use crate::random::{m_random, random_vector_on_sphere};
use crate::ray::Ray;
use crate::{
    hittable::HittableList,
    vec::{Point3, Vec3},
};

const SAMPLES_PER_PIXEL: i32 = 10;
const DEFAULT_MAX_RAY_RANGE: f64 = 100.0;
const MAX_DEPTH: i32 = 50;

pub struct Camera {
    aspect_ratio: f64,
    image_resolution: (i32, i32),
    viewport_uv: Vec3,
    pixel_delta_uv: Vec3,
    center: Point3,
    focal_length: f64,
    first_pixel: Point3,
}

impl Camera {
    pub fn new() -> Self {
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

        Camera {
            aspect_ratio,
            image_resolution: (image_width, image_height),
            viewport_uv,
            pixel_delta_uv,
            center: camera_center,
            focal_length,
            first_pixel,
        }
    }

    pub fn render(&self, world: &HittableList) -> std::io::Result<()> {
        let mut buffer = File::create("test.ppm")?;
        let (width, height) = self.image_resolution;
        let Self {
            first_pixel,
            pixel_delta_uv,
            center,
            ..
        } = self;
        buffer.write(
            format!(
                "P3\n{} {}\n255\n",
                self.image_resolution.0, self.image_resolution.1
            )
            .as_bytes(),
        )?;
        for row in 0..height {
            print!("\r{:4} / {:4}", row + 1, height);
            stdout().flush().unwrap();
            for col in 0..width {
                let mut color = Color::zero();
                for _ in 0..SAMPLES_PER_PIXEL {
                    let offset = Vec3::new(
                        col as f64 + m_random::<f64>() - 0.5,
                        row as f64 + m_random::<f64>() - 0.5,
                        0.0,
                    );
                    let current = *first_pixel + offset * *pixel_delta_uv;
                    let ray = Ray::new(*center, (current - *center).normalize());
                    color += Self::calc_ray(&ray, world, 0);
                }
                color = (color / SAMPLES_PER_PIXEL as f64).sqrt();
                buffer.write(write_color(color).as_bytes())?;
            }
        }
        println!("\ndone");
        Ok(())
    }

    fn calc_ray(ray: &Ray, world: &HittableList, depth: i32) -> Vec3 {
        if depth >= MAX_DEPTH {
            return Vec3::zero();
        }
        match world.hit(ray, (0.001, DEFAULT_MAX_RAY_RANGE)) {
            Some(result) => {
                let scatter_result = result.material.scatter(ray, &result);
                scatter_result.attenuation
                    * Self::calc_ray(
                        &Ray::new(result.p, scatter_result.scattered),
                        world,
                        depth + 1,
                    )
            }
            None => Color::mix(
                Color::one(),
                Color::new(0.5, 0.7, 1.0),
                ray.direction.y() * 0.5 + 0.5,
            ),
        }
    }
}
