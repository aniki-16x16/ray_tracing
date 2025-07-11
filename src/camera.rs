use std::io::Write;
use std::{fs::File, io::stdout};

use crate::color::{Color, write_color};
use crate::hittable::Hittable;
use crate::random::m_random;
use crate::ray::Ray;
use crate::{
    hittable::HittableList,
    vec::{Point3, Vec3},
};

const SAMPLES_PER_PIXEL: i32 = 50;
const DEFAULT_MAX_RAY_RANGE: f64 = 100.0;
const MAX_DEPTH: i32 = 50;

pub struct Camera {
    aspect_ratio: f64,
    image_resolution: (i32, i32),
    viewport_uv: (Vec3, Vec3),
    pixel_delta_uv: (Vec3, Vec3),
    center: Point3,
    focal_length: f64,
    first_pixel: Point3,
    vfov: f64,
    look_from: Vec3,
    look_at: Vec3,
    vup: Vec3,
    uvw: (Vec3, Vec3, Vec3),
}

impl Camera {
    pub fn new(vfov: f64, look_from: Point3, look_at: Point3, vup: Vec3) -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let image_width = 800;
        let image_height = (image_width as f64 / aspect_ratio).floor() as i32;
        let image_height = if image_height < 1 { 1 } else { image_height };

        let center = look_from;
        let focal_length = (look_from - look_at).length();
        let uvw = {
            let w = (look_from - look_at).normalize();
            let u = vup.cross(w);
            let v = w.cross(u);
            (u, v, w)
        };

        let viewport_height = 2.0 * (vfov / 2.0).to_radians().tan() * focal_length;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let viewport_u = viewport_width * uvw.0;
        let viewport_v = viewport_height * -uvw.1;
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let first_pixel = center - uvw.2 * focal_length - viewport_u / 2.0 - viewport_v / 2.0
            + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            aspect_ratio,
            image_resolution: (image_width, image_height),
            viewport_uv: (viewport_u, viewport_v),
            pixel_delta_uv: (pixel_delta_u, pixel_delta_v),
            center,
            focal_length,
            first_pixel,
            vfov,
            look_from,
            look_at,
            vup,
            uvw,
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
                    let offset = (
                        col as f64 + m_random::<f64>(),
                        row as f64 + m_random::<f64>(),
                    );
                    let current =
                        *first_pixel + offset.0 * pixel_delta_uv.0 + offset.1 * pixel_delta_uv.1;
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
            Some(result) => match result.material.scatter(ray, &result) {
                Some(scatter_result) => {
                    scatter_result.attenuation
                        * Self::calc_ray(
                            &Ray::new(result.p, scatter_result.scattered),
                            world,
                            depth + 1,
                        )
                }
                None => Color::zero(),
            },
            None => Color::mix(
                Color::one(),
                Color::new(0.5, 0.7, 1.0),
                ray.direction.y() * 0.5 + 0.5,
            ),
        }
    }
}
