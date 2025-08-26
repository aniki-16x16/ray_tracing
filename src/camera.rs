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

pub struct Camera {
    image_resolution: (u32, u32),
    pixel_delta_uv: (Vec3, Vec3),
    center: Point3,
    first_pixel: Point3,
    defocus_angle: f64,
    defocus_uv: (Vec3, Vec3),
    background: Color,
    samples_per_pixel: i32,
    max_ray_range: f64,
    max_depth: i32,
}

impl Camera {
    pub fn render(&self, world: &HittableList) {
        let (width, height) = self.image_resolution;
        let mut buffer = image::ImageBuffer::new(width, height);
        for row in 0..height {
            print!("\r{:4} / {:4}", row + 1, height);
            stdout().flush().unwrap();
            for col in 0..width {
                let mut color = Color::zero();
                for _ in 0..self.samples_per_pixel {
                    color += self.calc_ray(&self.get_ray(row, col), world, 0);
                }
                color = (color / self.samples_per_pixel as f64).sqrt();
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
        if depth >= self.max_depth {
            return Vec3::zero();
        }
        match world.hit(ray, Vec2::new(0.001, self.max_ray_range)) {
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

pub struct CameraBuilder {
    vfov: f64,
    look_from: Point3,
    look_at: Point3,
    vup: Vec3,
    focus_dist: f64,
    defocus_angle: f64,
    background: Color,
    aspect_ratio: f64,
    image_width: u32,
    samples_per_pixel: i32,
    max_ray_range: f64,
    max_depth: i32,
}
impl CameraBuilder {
    pub fn new(look_from: Point3, look_at: Point3) -> Self {
        Self {
            vfov: 45.0,
            look_from,
            look_at,
            vup: Vec3::new(0.0, 0.5, 0.0),
            focus_dist: (look_at - look_from).length(),
            defocus_angle: 0.0,
            background: Color::zero(),
            aspect_ratio: 16.0 / 9.0,
            image_width: 800,
            samples_per_pixel: 50,
            max_ray_range: 100.0,
            max_depth: 50,
        }
    }
    pub fn vertical_fov(mut self, vfov: f64) -> Self {
        self.vfov = vfov;
        self
    }
    pub fn view_up(mut self, vup: Vec3) -> Self {
        self.vup = vup;
        self
    }
    pub fn focus_dist(mut self, focus_dist: f64) -> Self {
        self.focus_dist = focus_dist;
        self
    }
    pub fn defocus_angle(mut self, defocus_angle: f64) -> Self {
        self.defocus_angle = defocus_angle;
        self
    }
    pub fn background_color(mut self, background: Color) -> Self {
        self.background = background;
        self
    }
    pub fn aspect_ratio(mut self, aspect_ratio: f64) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }
    pub fn image_width(mut self, image_width: u32) -> Self {
        self.image_width = image_width;
        self
    }
    pub fn samples_per_pixel(mut self, samples: i32) -> Self {
        self.samples_per_pixel = samples;
        self
    }
    pub fn max_ray_range(mut self, range: f64) -> Self {
        self.max_ray_range = range;
        self
    }
    pub fn max_depth(mut self, depth: i32) -> Self {
        self.max_depth = depth;
        self
    }

    pub fn build(self) -> Camera {
        let Self {
            aspect_ratio,
            image_width,
            look_from,
            look_at,
            vup,
            vfov,
            focus_dist,
            defocus_angle,
            background,
            samples_per_pixel,
            max_ray_range,
            max_depth,
        } = self;
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
            samples_per_pixel,
            max_ray_range,
            max_depth,
        }
    }
}
