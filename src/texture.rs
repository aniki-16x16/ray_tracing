use crate::{color::Color, vec::Vec2};

pub trait Texture {
    fn value(&self, uv: Vec2) -> Color;
}

pub struct SolidTexture {
    color: Color,
}

impl SolidTexture {
    pub fn new(color: Color) -> Self {
        SolidTexture { color }
    }
}

impl Texture for SolidTexture {
    fn value(&self, _: Vec2) -> Color {
        self.color
    }
}

pub struct CheckerTexture {
    scale: f64,
    color1: Color,
    color2: Color,
}

impl CheckerTexture {
    pub fn new(scale: f64) -> Self {
        CheckerTexture {
            scale,
            color1: Color::zero(),
            color2: Color::one(),
        }
    }

    pub fn with_color(scale: f64, color1: Color, color2: Color) -> Self {
        CheckerTexture {
            scale,
            color1,
            color2,
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, uv: Vec2) -> Color {
        let uv = uv * self.scale;
        if (uv.0.floor() + uv.1.floor()) % 2.0 == 0.0 {
            self.color1
        } else {
            self.color2
        }
    }
}
