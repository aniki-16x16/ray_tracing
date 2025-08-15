use std::sync::Arc;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    material::Material,
    math::get_sphere_uv,
    ray::Ray,
    vec::{Point3, Vec2, Vec3},
};

pub struct Sphere {
    center: Point3,
    target_center: Point3,
    radius: f64,
    material: Arc<dyn Material>,
    bbox: AABB,
}

impl Sphere {
    pub fn new(
        center: Point3,
        target_center: Point3,
        radius: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        Sphere {
            center,
            target_center,
            radius,
            material,
            bbox: AABB::from_aabb(
                &AABB::new(center - radius, center + radius),
                &AABB::new(target_center - radius, target_center + radius),
            ),
        }
    }
}

impl Hittable for Sphere {
    fn hit<'a>(&'a self, ray: &Ray, t_range: Vec2) -> Option<HitRecord<'a>> {
        let current_center = Vec3::mix(self.center, self.target_center, ray.time);
        let oc = current_center - ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let k = h * h - a * c;
        if k < 0.0 {
            None
        } else {
            let k = k.sqrt();
            let helper = |solve: f64| {
                if solve >= t_range.0 && solve <= t_range.1 {
                    let p = ray.at(solve);
                    let normal = (p - current_center).normalize();
                    let front_face = ray.direction.dot(normal) < 0.0;
                    Some(HitRecord {
                        p,
                        normal: if front_face { normal } else { -normal },
                        t: solve,
                        material: self.material.as_ref(),
                        front_face,
                        uv: get_sphere_uv(normal),
                    })
                } else {
                    None
                }
            };
            helper((h - k) / a).or(helper((h + k) / a))
        }
    }
    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
