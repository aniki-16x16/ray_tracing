use image::{DynamicImage, GenericImageView, ImageReader};

use crate::{
    color::Color,
    noise::PerlinNoise,
    vec::{Point3, Vec2},
};

pub trait Texture {
    fn value(&self, uv: Vec2, p: Point3) -> Color;
}

pub enum TextureEnum {
    SolidTexture(SolidTexture),
    CheckerTexture(CheckerTexture),
    NoiseTexture(NoiseTexture),
}

impl Texture for TextureEnum {
    fn value(&self, uv: Vec2, p: Point3) -> Color {
        match self {
            TextureEnum::SolidTexture(t) => t.value(uv, p),
            TextureEnum::CheckerTexture(t) => t.value(uv, p),
            TextureEnum::NoiseTexture(t) => t.value(uv, p),
        }
    }
}

impl Default for TextureEnum {
    fn default() -> Self {
        TextureEnum::SolidTexture(SolidTexture::new(Color::one()))
    }
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
    fn value(&self, _: Vec2, _: Point3) -> Color {
        self.color
    }
}

pub struct CheckerTexture {
    scale: Vec2,
    color1: Color,
    color2: Color,
}

impl CheckerTexture {
    pub fn new(scale: Vec2) -> Self {
        CheckerTexture {
            scale,
            color1: Color::zero(),
            color2: Color::one(),
        }
    }

    pub fn with_color(scale: Vec2, color1: Color, color2: Color) -> Self {
        CheckerTexture {
            scale,
            color1,
            color2,
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, uv: Vec2, _: Point3) -> Color {
        let uv = uv * self.scale;
        if (uv.0.floor() + uv.1.floor()) % 2.0 == 0.0 {
            self.color1
        } else {
            self.color2
        }
    }
}

pub struct ImageTexture {
    image: DynamicImage,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        let image = ImageReader::open(format!("texture/{filename}"))
            .expect("Failed to open image file.")
            .decode()
            .expect("Failed to decode image file.");
        ImageTexture { image }
    }
}

impl Texture for ImageTexture {
    fn value(&self, uv: Vec2, _: Point3) -> Color {
        // 如果图片没有加载成功，返回一个显眼的颜色用于调试
        if self.image.width() == 0 || self.image.height() == 0 {
            return Color::new(0.0, 1.0, 1.0); // 青色
        }
        let uv = Vec2::new(uv.0, 1.0 - uv.1);
        let pixel = self.image.get_pixel(
            (uv.0 * self.image.width() as f64) as u32,
            (uv.1 * self.image.height() as f64) as u32,
        );
        Color::new(pixel[0] as f64, pixel[1] as f64, pixel[2] as f64) / 255.0
    }
}

pub struct NoiseTexture {
    noise: PerlinNoise,
}

impl NoiseTexture {
    pub fn new() -> Self {
        NoiseTexture {
            noise: PerlinNoise::new(),
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _: Vec2, p: Point3) -> Color {
        let scale = 10.0;
        Color::from_single((10.0 * p.2 + scale * self.noise.turb(p, 7)).sin() * 0.5 + 0.5)
    }
}
