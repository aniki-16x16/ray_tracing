use crate::{
    color::Color,
    hittable::HitRecord,
    math::{reflect, refract, schlick_approx},
    random::{m_random, random_vector_on_sphere},
    ray::Ray,
    texture::{Texture, TextureEnum},
    vec::Vec3,
};

pub struct ScatterResult {
    pub attenuation: Color,
    pub scattered: Vec3,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<ScatterResult>;
    fn emit(&self) -> Color {
        Color::zero()
    }
}

pub enum MaterialEnum {
    Lambertian(Lambertian<TextureEnum>),
    Metal(Metal<TextureEnum>),
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight),
}

impl Material for MaterialEnum {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<ScatterResult> {
        match self {
            MaterialEnum::Lambertian(m) => m.scatter(ray, record),
            MaterialEnum::Metal(m) => m.scatter(ray, record),
            MaterialEnum::Dielectric(m) => m.scatter(ray, record),
            MaterialEnum::DiffuseLight(m) => m.scatter(ray, record),
        }
    }
    fn emit(&self) -> Color {
        match self {
            MaterialEnum::Lambertian(m) => m.emit(),
            MaterialEnum::Metal(m) => m.emit(),
            MaterialEnum::Dielectric(m) => m.emit(),
            MaterialEnum::DiffuseLight(m) => m.emit(),
        }
    }
}

pub struct Lambertian<T: Texture> {
    texture: T,
}

impl<T: Texture> Lambertian<T> {
    pub fn new(texture: T) -> Self {
        Lambertian { texture }
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, _ray: &Ray, record: &HitRecord) -> Option<ScatterResult> {
        let scatter_direction = record.normal + random_vector_on_sphere(record.normal);
        Some(ScatterResult {
            attenuation: self.texture.value(record.uv, record.p),
            scattered: scatter_direction.normalize(),
        })
    }
}

pub struct Metal<T: Texture> {
    texture: T,
    fuzz: f64,
}

impl<T: Texture> Metal<T> {
    pub fn new(texture: T, fuzz: f64) -> Self {
        Metal { texture, fuzz }
    }
}

impl<T: Texture> Material for Metal<T> {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<ScatterResult> {
        let mut scatter_direction = reflect(ray.direction, record.normal).normalize();
        scatter_direction += Vec3::random_rage(-1.0..1.0) * self.fuzz;
        if scatter_direction.dot(record.normal) > 0.0 {
            Some(ScatterResult {
                attenuation: self.texture.value(record.uv, record.p),
                scattered: scatter_direction.normalize(),
            })
        } else {
            None
        }
    }
}

pub struct Dielectric {
    eta: f64,
}

impl Dielectric {
    pub fn new(eta: f64) -> Self {
        Dielectric { eta }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<ScatterResult> {
        let ratio = if record.front_face {
            1.0 / self.eta
        } else {
            self.eta
        };
        let cos_theta = (-ray.direction).dot(record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let scatter_result =
            if ratio * sin_theta > 1.0 || m_random::<f64>() < schlick_approx(ratio, cos_theta) {
                reflect(ray.direction, record.normal)
            } else {
                refract(ratio, ray.direction, record.normal)
            };
        Some(ScatterResult {
            attenuation: Color::one(),
            scattered: scatter_result,
        })
    }
}

pub struct DiffuseLight {
    color: Color,
    strength: f64,
}

impl DiffuseLight {
    pub fn new(color: Color, strength: f64) -> Self {
        DiffuseLight { color, strength }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _: &Ray, _: &HitRecord) -> Option<ScatterResult> {
        None
    }
    fn emit(&self) -> Color {
        self.color * self.strength
    }
}
