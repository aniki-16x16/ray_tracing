use crate::vec::Vec3;

pub type Color = Vec3;

pub fn write_color(color: Color) -> String {
    let tmp = Vec3::new(
        (color.0 * 255.999).floor(),
        (color.1 * 255.999).floor(),
        (color.2 * 255.999).floor(),
    );
    format!("{} {} {}\n", tmp.0, tmp.1, tmp.2)
}

pub fn palette(a: Vec3, b: Vec3, c: Vec3, d: Vec3, t: f64) -> Color {
    let tmp = 6.283185 * (c * t + d);
    a + b * Vec3::new(tmp.0.cos(), tmp.1.cos(), tmp.2.cos())
}
