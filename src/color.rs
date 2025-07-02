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
