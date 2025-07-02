use crate::{
    color::Color, hittable::HitRecord, random::random_vector_on_sphere, ray::Ray, vec::Vec3,
};

pub struct ScatterResult {
    pub attenuation: Color,
    pub scattered: Vec3,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> ScatterResult;
}

pub struct Lambertian {
    color: Color,
}

impl Lambertian {
    pub fn new(color: Color) -> Self {
        Lambertian { color }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> ScatterResult {
        ScatterResult {
            attenuation: self.color,
            scattered: record.normal + random_vector_on_sphere(record.normal),
        }
    }
}

pub struct Metal {
    color: Color,
}

impl Metal {
    pub fn new(color: Color) -> Self {
        Metal { color }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> ScatterResult {
        ScatterResult {
            attenuation: self.color,
            scattered: ray.direction - record.normal * 2.0 * record.normal.dot(ray.direction),
        }
    }
}
