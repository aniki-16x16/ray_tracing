use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec::Point3,
};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> Option<HitRecord> {
        let oc = self.center.subtract(ray.origin());
        let a = 1.0;
        let h = ray.direction().dot(&oc);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let k = h * h - a * c;
        if k < 0.0 {
            None
        } else {
            let k = k.sqrt();
            let helper = |solve: f64| {
                if solve >= t_range.0 && solve <= t_range.1 {
                    let p = ray.at(solve);
                    Some(HitRecord::new(
                        p.clone(),
                        p.subtract(&self.center).normalize(),
                        solve,
                    ))
                } else {
                    None
                }
            };
            helper((h - k) / a).or(helper((h + k) / a))
        }
    }
}
