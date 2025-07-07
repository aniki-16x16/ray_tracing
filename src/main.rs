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
const CENTER_BALL_RADIUS: f64 = 0.5;

fn main() -> std::io::Result<()> {
    let mut world = HittableList::new();
    let ground = Sphere::new(
        Point3::new(0.0, -100.0 - 0.5, -FOCAL_LENGTH),
        100.0,
        Lambertian::new(Color::new(0.5, 0.6, 0.2)),
    );
    let center_ball = Sphere::new(
        Point3::new(0.0, 0.0, -FOCAL_LENGTH),
        CENTER_BALL_RADIUS,
        Lambertian::new(Color::new(0.7, 0.7, 0.7)),
    );
    let right_ball = Sphere::new(
        Point3::new(CENTER_BALL_RADIUS + 0.6, -0.5 + 0.6, -FOCAL_LENGTH),
        0.6,
        Dielectric::new(1.5),
    );
    let bubble = Sphere::new(
        Point3::new(CENTER_BALL_RADIUS + 0.6, -0.5 + 0.6, -FOCAL_LENGTH),
        0.5,
        Dielectric::new(1.0 / 1.5),
    );
    let left_ball = Sphere::new(
        Point3::new(-CENTER_BALL_RADIUS - 0.7, -0.5 + 0.7, -FOCAL_LENGTH),
        0.7,
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
