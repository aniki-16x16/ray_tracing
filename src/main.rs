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

fn main() -> std::io::Result<()> {
    let mut world = HittableList::new();
    world.push(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        Lambertian::new(Color::new(0.2, 0.2, 0.9)),
    ));
    world.push(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Lambertian::new(Color::new(0.5, 0.6, 0.2)),
    ));
    world.push(Sphere::new(
        Point3::new(1.1, 0.1, -1.0),
        0.6,
        Dielectric::new(1.5),
    ));
    world.push(Sphere::new(
        Point3::new(-1.2, 0.2, -1.0),
        0.7,
        Metal::new(Color::new(0.5, 0.1, 0.2), 0.0),
    ));

    let camera = Camera::new();
    camera.render(&world)?;

    Ok(())
}
