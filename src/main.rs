pub mod camera;
pub mod color;
pub mod geometry;
pub mod hittable;
pub mod math;
pub mod ray;
pub mod vec;

use crate::{camera::Camera, geometry::Sphere, hittable::HittableList, vec::Vec3};

fn main() -> std::io::Result<()> {
    let mut world = HittableList::new();
    world.push(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.push(Sphere::new(Vec3::new(0.0, -100.6, -1.0), 100.0));

    let camera = Camera::new();
    camera.render(&world)?;

    Ok(())
}
