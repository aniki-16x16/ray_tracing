pub mod camera;
pub mod color;
pub mod geometry;
pub mod hittable;
pub mod material;
pub mod math;
pub mod random;
pub mod ray;
pub mod vec;

use crate::{
    camera::Camera,
    color::Color,
    geometry::Sphere,
    hittable::HittableList,
    material::{Dielectric, Lambertian, Metal},
    vec::{Point3, Vec3},
};

fn main() -> std::io::Result<()> {
    let mut world = HittableList::new();
    let ground = Sphere::new(
        Point3::new(0.0, -100.5, 0.0),
        100.0,
        Lambertian::new(Color::new(0.1, 0.5, 0.2)),
    );
    let center_ball = Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        0.5,
        Lambertian::new(Color::new(0.8, 0.8, 0.8)),
    );
    let left_ball = Sphere::new(Point3::new(-1.0, 0.0, 0.0), 0.5, Dielectric::new(1.5));
    let bubble = Sphere::new(Point3::new(-1.0, 0.0, 0.0), 0.3, Dielectric::new(1.0 / 1.5));
    let right_ball = Sphere::new(
        Point3::new(1.0, 0.0, 0.0),
        0.5,
        Metal::new(Color::new(0.2, 0.5, 0.8), 0.2),
    );
    world
        .push(ground)
        .push(center_ball)
        .push(left_ball)
        .push(bubble)
        .push(right_ball);

    let vup = Vec3::new(0.0, 0.5, 0.0);
    let camera = Camera::new(
        60.0,
        Point3::new(-3.0, 3.0, 2.0),
        Point3::zero(),
        vup,
        5.0,
        10.0,
    );
    camera.render(&world)?;

    Ok(())
}
