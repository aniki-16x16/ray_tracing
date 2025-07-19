use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vec::{Point3, Vec2, Vec3},
};

pub struct Sphere {
    center: Point3,
    target_center: Point3,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(
        center: Point3,
        target_center: Point3,
        radius: f64,
        material: Box<dyn Material>,
    ) -> Self {
        Sphere {
            center,
            target_center,
            radius,
            material,
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
                    Some(HitRecord::new(
                        p,
                        if front_face { normal } else { -normal },
                        solve,
                        self.material.as_ref(),
                        front_face,
                    ))
                } else {
                    None
                }
            };
            helper((h - k) / a).or(helper((h + k) / a))
        }
    }
}
