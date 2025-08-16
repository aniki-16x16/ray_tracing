use std::sync::Arc;

use crate::{
    color::Color,
    hittable::HitRecord,
    math::{reflect, refract, schlick_approx},
    random::{m_random, random_vector_on_sphere},
    ray::Ray,
    texture::Texture,
    vec::Vec3,
};

pub struct ScatterResult {
    pub attenuation: Color,
    pub scattered: Vec3,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<ScatterResult>;
}

pub struct Lambertian {
    texture: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(texture: impl Texture + 'static) -> Self {
        Lambertian {
            texture: Arc::new(texture),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, record: &HitRecord) -> Option<ScatterResult> {
        let scatter_direction = record.normal + random_vector_on_sphere(record.normal);
        Some(ScatterResult {
            attenuation: self.texture.value(record.uv, record.p),
            scattered: scatter_direction.normalize(),
        })
    }
}

pub struct Metal {
    texture: Arc<dyn Texture>,
    fuzz: f64,
}

impl Metal {
    pub fn new(texture: impl Texture + 'static, fuzz: f64) -> Self {
        Metal {
            texture: Arc::new(texture),
            fuzz,
        }
    }
}

impl Material for Metal {
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
