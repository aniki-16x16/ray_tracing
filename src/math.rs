use crate::vec::Vec3;

pub fn mix(a: f64, b: f64, t: f64) -> f64 {
    (1.0 - t) * a + t * b
}

pub fn reflect(direction: Vec3, normal: Vec3) -> Vec3 {
    direction - 2.0 * direction.dot(normal) * normal
}

pub fn refract(ratio: f64, direc: Vec3, normal: Vec3) -> Vec3 {
    let cos_theta = -direc.dot(normal);
    let perp = ratio * (direc - cos_theta * normal);
    let para = -(1.0 - perp.length_squared()).abs().sqrt() * normal;
    perp + para
}

pub fn schlick_approx(ratio: f64, cos_theta: f64) -> f64 {
    let mut r0 = (1.0 - ratio) / (1.0 + ratio);
    r0 *= r0;
    r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
}
