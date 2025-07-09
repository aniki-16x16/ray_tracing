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
    vec::Point3,
};

const FOCAL_LENGTH: f64 = 1.0;

fn main() -> std::io::Result<()> {
    let mut world = HittableList::new();
    let ground = Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Lambertian::new(Color::new(0.5, 0.6, 0.2)),
    );
    let center_ball = Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        Lambertian::new(Color::new(0.9, 0.9, 0.9)),
    );
    let right_ball = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, Dielectric::new(1.5));
    let bubble = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.4, Dielectric::new(1.0 / 1.5));
    let left_ball = Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Metal::new(Color::new(0.5, 0.1, 0.2), 0.2),
    );
    world.push(center_ball);
    world.push(ground);
    world.push(right_ball);
    world.push(bubble);
    world.push(left_ball);

    let camera = Camera::new(FOCAL_LENGTH);
    camera.render(&world)?;

    Ok(())
}
