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
    camera::CameraBuilder,
    color::Color,
    geometry::Quad,
    hittable::HittableList,
    material::{DiffuseLight, Lambertian},
    texture::SolidTexture,
    vec::{Point3, Vec3},
};

fn main() {
    let mut world = HittableList::new();
    let red = Arc::new(Lambertian::new(SolidTexture::new(Color::new(
        0.8, 0.1, 0.1,
    ))));
    let green = Arc::new(Lambertian::new(SolidTexture::new(Color::new(
        0.1, 0.8, 0.1,
    ))));
    let white = Arc::new(Lambertian::new(SolidTexture::new(Color::new(
        0.8, 0.8, 0.8,
    ))));
    let box_size = 555.0;
    world
        .push(Quad::new(
            Point3::new(box_size, 0.0, 0.0),
            Vec3::from_axis_y(box_size),
            Vec3::from_axis_z(box_size),
            red.clone(),
        ))
        .push(Quad::new(
            Point3::new(0.0, box_size, 0.0),
            Vec3::from_axis_x(box_size),
            Vec3::from_axis_z(box_size),
            white.clone(),
        ))
        .push(Quad::new(
            Point3::new(0.0, 0.0, box_size),
            Vec3::from_axis_x(box_size),
            Vec3::from_axis_y(box_size),
            white.clone(),
        ))
        .push(Quad::new(
            Point3::zero(),
            Vec3::from_axis_x(box_size),
            Vec3::from_axis_z(box_size),
            white.clone(),
        ))
        .push(Quad::new(
            Point3::zero(),
            Vec3::from_axis_y(box_size),
            Vec3::from_axis_z(box_size),
            green.clone(),
        ))
        .push(Quad::new(
            Point3::new(box_size * 0.4, box_size - 1.0, box_size * 0.4),
            Vec3::from_axis_x(box_size * 0.2),
            Vec3::from_axis_z(box_size * 0.2),
            Arc::new(DiffuseLight::new(Color::one(), 20.0)),
        ));

    let mut world_bvh = HittableList::new();
    world_bvh.push(BvhNode::new(&mut world.list));

    let look_from = Point3::new(box_size * 0.5, box_size * 0.5, -box_size);
    let look_at = Point3::new(box_size * 0.5, box_size * 0.5, box_size);
    let camera = CameraBuilder::new(look_from, look_at)
        .image_width(1920)
        .vertical_fov(90.0)
        .samples_per_pixel(200)
        .max_ray_range(2000.0)
        .build();
    let start_time = Instant::now();
    camera.render(&world_bvh);
    let elapsed_time = start_time.elapsed();
    println!("耗时{}秒", elapsed_time.as_secs_f64());
}
