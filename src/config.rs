use serde::Deserialize;

use crate::bvh::BvhNode;
use crate::camera::CameraBuilder;
use crate::color::Color;
use crate::geometry::{ConstantMedium, Cube, GeometryEnum, Quad, RotateY, Sphere, Translate};
use crate::hittable::HittableList;
use crate::material::{Dielectric, DiffuseLight, Isotropic, Lambertian, MaterialEnum, Metal};
use crate::texture::{CheckerTexture, NoiseTexture, SolidTexture, TextureEnum};
use crate::vec::{Point3, Vec2, Vec3};
use std::fs;
use std::sync::Arc;

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)] // 如果 toml 中有未知字段，则报错，有助于调试
pub struct Config {
    #[serde(default)]
    pub camera: Option<CameraConfig>,
    #[serde(default)]
    pub objects: Vec<GeometryConfig>,
}

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct CameraConfig {
    pub look_from: Option<Point3>,
    pub look_at: Option<Point3>,
    pub vup: Option<Vec3>,
    pub vertical_fov: Option<f64>,
    pub image_width: Option<u32>,
    pub aspect_ratio: Option<f64>,
    pub defocus_angle: Option<f64>,
    pub focus_dist: Option<f64>,
    pub samples_per_pixel: Option<i32>,
    pub max_depth: Option<i32>,
    pub max_ray_range: Option<f64>,
    pub background_color: Option<Color>,
}

pub fn load_config_from_file(path: &str) -> Config {
    match fs::read_to_string(path) {
        Ok(contents) => {
            toml::from_str(&contents).expect("配置文件 config.toml 解析失败！请检查格式。")
        }
        Err(_) => {
            println!("未找到配置文件 '{}'。将使用默认设置。", path);
            Config::default()
        }
    }
}

pub trait Configurable {
    fn apply_config(self, config: &CameraConfig) -> Self;
}
impl Configurable for CameraBuilder {
    fn apply_config(mut self, config: &CameraConfig) -> Self {
        if let Some(look_from) = config.look_from {
            self = self.look_from(look_from);
        }
        if let Some(look_at) = config.look_at {
            self = self.look_at(look_at);
        }
        if let Some(vup) = config.vup {
            self = self.view_up(vup);
        }
        if let Some(fov) = config.vertical_fov {
            self = self.vertical_fov(fov);
        }
        if let Some(width) = config.image_width {
            self = self.image_width(width);
        }
        if let Some(ratio) = config.aspect_ratio {
            self = self.aspect_ratio(ratio);
        }
        if let Some(angle) = config.defocus_angle {
            self = self.defocus_angle(angle);
        }
        if let Some(dist) = config.focus_dist {
            self = self.focus_dist(dist);
        } else {
            let dist = (self.look_at - self.look_from).length();
            self = self.focus_dist(dist);
        }
        if let Some(samples) = config.samples_per_pixel {
            self = self.samples_per_pixel(samples);
        }
        if let Some(depth) = config.max_depth {
            self = self.max_depth(depth);
        }
        if let Some(range) = config.max_ray_range {
            self = self.max_ray_range(range);
        }
        if let Some(bg) = config.background_color {
            self = self.background_color(bg);
        }
        self
    }
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TextureConfig {
    Solid {
        color: Option<Color>,
    },
    Checker {
        scale: Option<Vec2>,
        color1: Option<Color>,
        color2: Option<Color>,
    },
    Noise {},
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MaterialConfig {
    Lambertian {
        texture: Option<TextureConfig>,
    },
    Metal {
        texture: Option<TextureConfig>,
        fuzz: Option<f64>,
    },
    Dielectric {
        eta: f64,
    },
    DiffuseLight {
        color: Option<Color>,
        strength: f64,
    },
    Isotropic {
        texture: Option<TextureConfig>,
    },
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum GeometryConfig {
    Sphere {
        center: Point3,
        target_center: Option<Point3>,
        radius: f64,
        material: Option<MaterialConfig>,
    },
    Quad {
        q: Point3,
        u: Vec3,
        v: Vec3,
        material: Option<MaterialConfig>,
    },
    Cube {
        a: Point3,
        b: Point3,
        material: Option<MaterialConfig>,
    },
    Translate {
        instance: Box<GeometryConfig>,
        offset: Vec3,
    },
    RotateY {
        instance: Box<GeometryConfig>,
        angle: f64,
    },
    ConstantMedium {
        boundary: Box<GeometryConfig>,
        density: f64,
        texture: TextureConfig,
    },
}

fn build_texture(config: TextureConfig) -> TextureEnum {
    match config {
        TextureConfig::Solid { color } => {
            TextureEnum::SolidTexture(SolidTexture::new(color.unwrap_or(Color::from_single(0.8))))
        }
        TextureConfig::Checker {
            scale,
            color1,
            color2,
        } => TextureEnum::CheckerTexture(CheckerTexture::with_color(
            scale.unwrap_or(Vec2::from_single(2.0)),
            color1.unwrap_or(Color::zero()),
            color2.unwrap_or(Color::one()),
        )),
        TextureConfig::Noise {} => TextureEnum::NoiseTexture(NoiseTexture::new()),
    }
}

fn build_material(config: MaterialConfig) -> Arc<MaterialEnum> {
    let texture_helper = |texture: Option<TextureConfig>| {
        texture.map_or_else(|| TextureEnum::default(), |t| build_texture(t))
    };
    match config {
        MaterialConfig::Lambertian { texture } => Arc::new(MaterialEnum::Lambertian(
            Lambertian::new(texture_helper(texture)),
        )),
        MaterialConfig::Metal { texture, fuzz } => Arc::new(MaterialEnum::Metal(Metal::new(
            texture_helper(texture),
            fuzz.unwrap_or(0.0),
        ))),
        MaterialConfig::Dielectric { eta } => {
            Arc::new(MaterialEnum::Dielectric(Dielectric::new(eta)))
        }
        MaterialConfig::DiffuseLight { color, strength } => Arc::new(MaterialEnum::DiffuseLight(
            DiffuseLight::new(color.unwrap_or(Vec3::one()), strength),
        )),
        MaterialConfig::Isotropic { texture } => Arc::new(MaterialEnum::Isotropic(Isotropic::new(
            texture_helper(texture),
        ))),
    }
}

fn build_geometry(config: GeometryConfig) -> GeometryEnum {
    let material_helper = |material: Option<MaterialConfig>| {
        material.map_or_else(|| Arc::new(MaterialEnum::default()), |m| build_material(m))
    };
    match config {
        GeometryConfig::Sphere {
            center,
            target_center,
            radius,
            material,
        } => GeometryEnum::Sphere(Sphere::new(
            center,
            target_center.unwrap_or(center),
            radius,
            material_helper(material),
        )),
        GeometryConfig::Quad { q, u, v, material } => {
            GeometryEnum::Quad(Quad::new(q, u, v, material_helper(material)))
        }
        GeometryConfig::Cube { a, b, material } => {
            GeometryEnum::Cube(Cube::new(a, b, material_helper(material)))
        }
        GeometryConfig::Translate { instance, offset } => {
            GeometryEnum::Translate(Translate::new(build_geometry(*instance), offset))
        }
        GeometryConfig::RotateY { instance, angle } => {
            GeometryEnum::RotateY(RotateY::new(build_geometry(*instance), angle))
        }
        GeometryConfig::ConstantMedium {
            boundary,
            density,
            texture,
        } => GeometryEnum::ConstantMedium(ConstantMedium::new(
            build_geometry(*boundary),
            density,
            build_texture(texture),
        )),
    }
}

pub fn build_world(config: Vec<GeometryConfig>) -> HittableList {
    let mut world = HittableList::new();
    for item in config {
        world.push(build_geometry(item));
    }
    let mut world_bvh = HittableList::new();
    world_bvh.push(BvhNode::new(&mut world.list));
    world_bvh
}
