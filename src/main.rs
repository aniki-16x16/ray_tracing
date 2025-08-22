pub mod aabb;
pub mod bvh;
pub mod camera;
pub mod color;
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
    bvh::BvhNode,
    camera::Camera,
    color::Color,
    geometry::{Quad, Sphere},
    hittable::HittableList,
    material::{DiffuseLight, Lambertian},
    texture::NoiseTexture,
    vec::{Point3, Vec3},
};

fn main() {
    let mut world = HittableList::new();
    let ground = Quad::new(
        Point3::new(-50.0, -1.0, 50.0),
        Vec3::new(100.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -100.0),
        Arc::new(Lambertian::new(NoiseTexture::new())),
    );
    let ball = Sphere::new(
        Point3::zero(),
        Point3::zero(),
        1.0,
        Arc::new(Lambertian::new(NoiseTexture::new())),
    );
    let light = Quad::new(
        Point3::new(2.0, -0.9, 1.0),
        Vec3::new(0.0, 0.0, -2.0),
        Vec3::new(0.0, 1.8, 0.0),
        Arc::new(DiffuseLight::new(Color::one(), 10.0)),
    );
    let top_light = Sphere::new(
        Point3::new(0.0, 3.0, 0.0),
        Point3::new(0.0, 3.0, 0.0),
        1.5,
        Arc::new(DiffuseLight::new(Color::new(0.8, 0.8, 0.3), 20.0)),
    );
    world.push(ground).push(ball).push(light).push(top_light);
    let mut world_bvh = HittableList::new();
    world_bvh.push(BvhNode::new(&mut world.list));

    let vup = Vec3::new(0.0, 0.5, 0.0);
    let look_from = Point3::new(0.0, 1.0, 4.0);
    let look_at = Point3::new(0.0, 0.5, 0.0);
    let camera = Camera::new(
        90.0,
        look_from,
        look_at,
        vup,
        (look_at - look_from).length(),
        0.0,
        Color::new(0.0, 0.01, 0.05),
    );
    let start_time = Instant::now();
    camera.render(&world_bvh);
    let elapsed_time = start_time.elapsed();
    println!("耗时{}秒", elapsed_time.as_secs_f64());
}
