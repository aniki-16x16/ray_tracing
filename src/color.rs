use crate::vec::Vec3;

pub type Color = Vec3;

pub fn write_color(color: Color) -> String {
    let tmp = Vec3::new(
        (color.x() * 255.999).floor(),
        (color.y() * 255.999).floor(),
        (color.z() * 255.999).floor(),
    );
    format!("{} {} {}\n", tmp.x(), tmp.y(), tmp.z())
}

pub fn palette(a: Vec3, b: Vec3, c: Vec3, d: Vec3, t: f64) -> Color {
    let tmp = 6.283185 * (c * t + d);
    a + b * Vec3::new(tmp.x().cos(), tmp.y().cos(), tmp.z().cos())
}
