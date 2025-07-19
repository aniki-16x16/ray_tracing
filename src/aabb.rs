use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec::{Point3, Vec2},
};

pub struct AABB {
    x_interval: Vec2,
    y_interval: Vec2,
    z_interval: Vec2,
}

impl AABB {
    pub fn new(a: Point3, b: Point3) -> Self {
        AABB {
            x_interval: Vec2::new(a.0.min(b.0), a.0.max(b.0)),
            y_interval: Vec2::new(a.1.min(b.1), a.1.max(b.1)),
            z_interval: Vec2::new(a.2.min(b.2), a.2.max(b.2)),
        }
    }

    fn hit(&self, ray: &Ray, t_range: Vec2) -> bool {
        let mut result = t_range;
        for idx in 0..3 {
            let origin = [ray.origin.0, ray.origin.1, ray.origin.2][idx];
            let direc = [ray.direction.0, ray.direction.1, ray.direction.2][idx];
            let interval = [self.x_interval, self.y_interval, self.z_interval][idx];
            let mut t0 = result.0.max((interval.0 - origin) / direc);
            let mut t1 = result.1.min((interval.1 - origin) / direc);
            if t0 > t1 {
                std::mem::swap(&mut t0, &mut t1);
            }
            result.0 = result.0.max(t0);
            result.1 = result.1.min(t1);
            if result.0 >= result.1 {
                return false;
            }
        }
        true
    }
}
