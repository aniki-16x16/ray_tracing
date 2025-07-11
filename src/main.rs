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
    color::{Color, palette},
    geometry::Sphere,
    hittable::HittableList,
    material::{Dielectric, Lambertian, Material, Metal},
    random::{m_random, m_random_range},
    vec::{Point3, Vec3},
};

const GRID_SIZE: i32 = 10;

fn main() -> std::io::Result<()> {
    let palette_helper = |t: f64| -> Color {
        let params: (Vec3, Vec3, Vec3, Vec3) = (
            Vec3::new(0.5, 0.5, 0.5),
            Vec3::new(0.5, 0.5, 0.5),
            Vec3::new(1.0, 1.0, 1.0),
            Vec3::new(0.0, 0.1, 0.2),
        );
        palette(params.0, params.1, params.2, params.3, t)
    };

    let mut world = HittableList::new();
    let ground = Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(Lambertian::new(Color::new(0.6, 0.6, 0.6))),
    );
    let center_ball = Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Box::new(Dielectric::new(1.5)),
    );
    let bubble = Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        0.8,
        Box::new(Dielectric::new(1.0 / 1.5)),
    );
    let left_ball = Sphere::new(
        Point3::new(-2.0, 1.0, 0.0),
        1.0,
        Box::new(Lambertian::new(palette_helper(0.1))),
    );
    let right_ball = Sphere::new(
        Point3::new(2.0, 1.0, 0.0),
        1.0,
        Box::new(Metal::new(palette_helper(0.1), 0.0)),
    );
    world
        .push(ground)
        .push(center_ball)
        .push(left_ball)
        .push(bubble)
        .push(right_ball);
    for i in -GRID_SIZE..GRID_SIZE {
        for j in -GRID_SIZE..GRID_SIZE {
            if i == 0 && (j.abs() == 2 || j == 0) {
                continue;
            }
            let radius = m_random_range(0.05..0.3);
            let offset = 0.5 - radius;
            let center = Vec3::new(
                j as f64 + m_random_range(-offset..offset),
                radius,
                i as f64 + m_random_range(-offset..offset),
            );
            let mat_chance = m_random::<f64>();
            let mat: Box<dyn Material> = if mat_chance < 0.6 {
                Box::new(Lambertian::new(palette_helper(
                    center.x() / GRID_SIZE as f64,
                )))
            } else if mat_chance < 0.95 {
                Box::new(Metal::new(
                    palette_helper(center.x() / GRID_SIZE as f64),
                    m_random::<f64>() * 0.3,
                ))
            } else {
                Box::new(Dielectric::new(1.5))
            };
            world.push(Sphere::new(center, radius, mat));
        }
    }

    let vup = Vec3::new(0.0, 0.5, 0.0);
    let look_from = Point3::new(13.0, 2.0, 9.0);
    let look_at = Point3::new(0.0, 1.0, 0.0);
    let camera = Camera::new(
        30.0,
        look_from,
        look_at,
        vup,
        (look_at - look_from).length(),
        1.0,
    );
    camera.render(&world)?;

    Ok(())
}
