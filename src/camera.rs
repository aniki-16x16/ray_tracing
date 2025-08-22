use std::io::Write;
use std::io::stdout;

use crate::color::{Color, write_color};
use crate::hittable::Hittable;
use crate::random::{m_random, random_in_disk};
use crate::ray::Ray;
use crate::vec::Vec2;
use crate::{
    hittable::HittableList,
    vec::{Point3, Vec3},
};

const SAMPLES_PER_PIXEL: i32 = 50;
const DEFAULT_MAX_RAY_RANGE: f64 = 100.0;
const MAX_DEPTH: i32 = 50;

pub struct Camera {
    image_resolution: (u32, u32),
    pixel_delta_uv: (Vec3, Vec3),
    center: Point3,
    first_pixel: Point3,
    defocus_angle: f64,
    defocus_uv: (Vec3, Vec3),
    background: Color,
}

impl Camera {
    pub fn new(
        vfov: f64,
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
        focus_dist: f64,
        defocus_angle: f64,
        background: Color,
    ) -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let image_width = 800;
        let image_height = (image_width as f64 / aspect_ratio).floor() as u32;
        let image_height = if image_height < 1 { 1 } else { image_height };

        let center = look_from;
        let uvw = {
            let w = (look_from - look_at).normalize();
            let u = vup.cross(w);
            let v = w.cross(u);
            (u, v, w)
        };

        let viewport_height = 2.0 * (vfov / 2.0).to_radians().tan() * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let viewport_u = viewport_width * uvw.0;
        let viewport_v = viewport_height * -uvw.1;
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;
        let first_pixel = center - uvw.2 * focus_dist - viewport_u / 2.0 - viewport_v / 2.0
            + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius = (defocus_angle / 2.0).to_radians().tan() * focus_dist;

        Camera {
            image_resolution: (image_width, image_height),
            pixel_delta_uv: (pixel_delta_u, pixel_delta_v),
            center,
            first_pixel,
            defocus_angle,
            defocus_uv: (defocus_radius * uvw.0, defocus_radius * uvw.1),
            background,
        }
    }

    pub fn render(&self, world: &HittableList) {
        let (width, height) = self.image_resolution;
        let mut buffer = image::ImageBuffer::new(width, height);
        for row in 0..height {
            print!("\r{:4} / {:4}", row + 1, height);
            stdout().flush().unwrap();
            for col in 0..width {
                let mut color = Color::zero();
                for _ in 0..SAMPLES_PER_PIXEL {
                    color += self.calc_ray(&self.get_ray(row, col), world, 0);
                }
                color = (color / SAMPLES_PER_PIXEL as f64).sqrt();
                let pixel = buffer.get_pixel_mut(col, row);
                *pixel = image::Rgb(write_color(color));
            }
        }
        buffer
            .save("output.png")
            .expect("Failed to save render result.");
        println!("\ndone");
    }

    fn calc_ray(&self, ray: &Ray, world: &HittableList, depth: i32) -> Vec3 {
        if depth >= MAX_DEPTH {
            return Vec3::zero();
        }
        match world.hit(ray, Vec2::new(0.001, DEFAULT_MAX_RAY_RANGE)) {
            Some(result) => match result.material.scatter(ray, &result) {
                Some(scatter_result) => {
                    scatter_result.attenuation
                        * self.calc_ray(
                            &Ray::new(result.p, scatter_result.scattered, ray.time),
                            world,
                            depth + 1,
                        )
                        + result.material.emit()
                }
                None => result.material.emit(),
            },
            None => self.background,
        }
    }

    fn get_ray(&self, row: u32, col: u32) -> Ray {
        let Self {
            pixel_delta_uv,
            center,
            first_pixel,
            defocus_angle,
            defocus_uv,
            ..
        } = self;
        let pixel_offset = Vec2::new(col as f64, row as f64) + m_random::<Vec2>();
        let pixel_current =
            *first_pixel + pixel_offset.0 * pixel_delta_uv.0 + pixel_offset.1 * pixel_delta_uv.1;
        let ray_current = if *defocus_angle <= 0.0 {
            *center
        } else {
            let offset = random_in_disk();
            *center + defocus_uv.0 * offset.0 + defocus_uv.1 * offset.1
        };
        Ray::new(
            ray_current,
            (pixel_current - ray_current).normalize(),
            m_random::<f64>(),
        )
    }
}
