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
    geometry::Sphere,
    hittable::HittableList,
    material::Lambertian,
    texture::NoiseTexture,
    vec::{Point3, Vec3},
};

fn main() {
    let mut world = HittableList::new();
    let ball_bottom = Sphere::new(
        Point3::new(0.0, -100.0, 0.0),
        Point3::new(0.0, -100.0, 0.0),
        100.0,
        Arc::new(Lambertian::new(NoiseTexture::new())),
    );
    let ball_top = Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(NoiseTexture::new())),
    );
    world.push(ball_bottom).push(ball_top);
    let mut world_bvh = HittableList::new();
    world_bvh.push(BvhNode::new(&mut world.list));

    let vup = Vec3::new(0.0, 0.5, 0.0);
    let look_from = Point3::new(3.0, 4.0, 15.0);
    let look_at = Point3::new(0.0, 1.0, 0.0);
    let camera = Camera::new(
        60.0,
        look_from,
        look_at,
        vup,
        (look_at - look_from).length(),
        1.0,
    );
    let start_time = Instant::now();
    camera.render(&world_bvh);
    let elapsed_time = start_time.elapsed();
    println!("耗时{}秒", elapsed_time.as_secs_f64());
}
