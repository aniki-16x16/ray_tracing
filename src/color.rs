use crate::vec::Vec3;

pub type Color = Vec3;

pub fn write_color(color: Color) -> [u8; 3] {
    [
        (color.0.clamp(0.0, 1.0) * 255.999).floor() as u8,
        (color.1.clamp(0.0, 1.0) * 255.999).floor() as u8,
        (color.2.clamp(0.0, 1.0) * 255.999).floor() as u8,
    ]
}

pub fn palette(a: Vec3, b: Vec3, c: Vec3, d: Vec3, t: f64) -> Color {
    let tmp = 6.283185 * (c * t + d);
    a + b * Vec3::new(tmp.0.cos(), tmp.1.cos(), tmp.2.cos())
}
