pub mod aabb;
pub mod bvh;
pub mod camera;
pub mod color;
pub mod config;
pub mod geometry;
pub mod hittable;
pub mod material;
pub mod math;
pub mod noise;
pub mod random;
pub mod ray;
pub mod texture;
pub mod vec;

use std::{sync::Arc, time::Instant};

use crate::{
    camera::CameraBuilder,
    color::Color,
    config::{Configurable, build_world, load_config_from_file},
    geometry::{Quad, Sphere},
    hittable::HittableList,
    material::Lambertian,
    texture::SolidTexture,
    vec::{Point3, Vec3},
};

fn main() {
    let config = load_config_from_file("config.toml");
    let mut camera_builder = CameraBuilder::new();
    if let Some(camera_config) = &config.camera {
        camera_builder = camera_builder.apply_config(camera_config);
    }
    let camera = camera_builder.build();
    let mut world = HittableList::new();
    if config.objects.len() > 0 {
        world = build_world(&config.objects);
    } else {
        world
            .push(Sphere::new(
                Point3::zero(),
                Point3::zero(),
                0.5,
                Arc::new(Lambertian::new(SolidTexture::new(Color::from_single(0.8)))),
            ))
            .push(Quad::new(
                Point3::new(-100.0, -0.5, 100.0),
                Vec3::from_axis_x(200.0),
                Vec3::from_axis_z(-200.0),
                Arc::new(Lambertian::new(SolidTexture::new(Color::from_single(0.6)))),
            ));
    }
    let start_time = Instant::now();
    camera.render(&world);
    let elapsed_time = start_time.elapsed();
    println!("耗时{}秒", elapsed_time.as_secs_f64());
}
