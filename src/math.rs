use std::f64::consts::PI;

use crate::vec::{Point3, Vec2, Vec3};

pub fn mix(a: f64, b: f64, t: f64) -> f64 {
    (1.0 - t) * a + t * b
}

pub fn remap_00(a: f64, b: f64, x: f64) -> f64 {
    (x - a) / (b - a)
}

pub fn remap_01(v0: (f64, f64), v1: (f64, f64), x: f64) -> f64 {
    mix(v1.0, v1.1, remap_00(v0.0, v0.1, x))
}

pub fn reflect(direction: Vec3, normal: Vec3) -> Vec3 {
    direction - 2.0 * direction.dot(normal) * normal
}

pub fn refract(ratio: f64, direc: Vec3, normal: Vec3) -> Vec3 {
    let cos_theta = (-direc).dot(normal).min(1.0);
    let perp = ratio * (direc + cos_theta * normal);
    let para = -(1.0 - perp.length_squared()).sqrt() * normal;
    perp + para
}

pub fn schlick_approx(ratio: f64, cos_theta: f64) -> f64 {
    let mut r0 = (1.0 - ratio) / (1.0 + ratio);
    r0 *= r0;
    r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
}

pub fn get_sphere_uv(p: Point3) -> Vec2 {
    let theta = (-p.1).acos();
    let phi = p.2.atan2(p.0) + PI;
    Vec2::new(phi * 0.5 / PI, theta / PI)
}
