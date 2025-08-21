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
    material::{Lambertian, Metal},
    texture::{NoiseTexture, SolidTexture},
    vec::{Point3, Vec3},
};

fn main() {
    let mut world = HittableList::new();
    let plane_left = Quad::new(
        Point3::new(-1.0, -1.0, 1.0),
        Vec3::new(0.0, 0.0, -2.0),
        Vec3::new(0.0, 2.0, 0.0),
        Arc::new(Metal::new(
            SolidTexture::new(Color::new(0.6, 0.3, 0.3)),
            0.0,
        )),
    );
    let plane_right = Quad::new(
        Point3::new(1.0, -1.0, 1.0),
        Vec3::new(0.0, 0.0, -2.0),
        Vec3::new(0.0, 2.0, 0.0),
        Arc::new(Lambertian::new(SolidTexture::new(Color::new(
            0.1, 0.7, 0.1,
        )))),
    );
    let plane_bottom = Quad::new(
        Point3::new(-1.0, -1.0, 1.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -2.0),
        Arc::new(Lambertian::new(SolidTexture::new(Color::new(
            0.6, 0.6, 0.6,
        )))),
    );
    let plane_top = Quad::new(
        Point3::new(-1.0, 1.0, 1.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -2.0),
        Arc::new(Metal::new(
            SolidTexture::new(Color::new(0.1, 0.1, 0.7)),
            0.2,
        )),
    );
    let ball = Sphere::new(
        Point3::zero(),
        Point3::zero(),
        0.8,
        Arc::new(Lambertian::new(NoiseTexture::new())),
    );
    world
        .push(plane_left)
        .push(plane_right)
        .push(plane_bottom)
        .push(plane_top)
        .push(ball);
    let mut world_bvh = HittableList::new();
    world_bvh.push(BvhNode::new(&mut world.list));

    let vup = Vec3::new(0.0, 0.5, 0.0);
    let look_from = Point3::new(5.0, 0.5, 4.0);
    let look_at = Point3::new(-0.3, 0.0, 0.0);
    let camera = Camera::new(
        45.0,
        look_from,
        look_at,
        vup,
        (look_at - look_from).length(),
        0.0,
    );
    let start_time = Instant::now();
    camera.render(&world_bvh);
    let elapsed_time = start_time.elapsed();
    println!("耗时{}秒", elapsed_time.as_secs_f64());
}
