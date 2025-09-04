use std::ops::Index;

use crate::{
    ray::Ray,
    vec::{Point3, Vec2, Vec3},
};

#[derive(Debug, Default, Clone)]
pub struct AABB {
    x_interval: Vec2,
    y_interval: Vec2,
    z_interval: Vec2,
}

impl Index<usize> for AABB {
    type Output = Vec2;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x_interval,
            1 => &self.y_interval,
            2 => &self.z_interval,
            _ => panic!("Index out of bounds for AABB"),
        }
    }
}

impl AABB {
    pub fn new(a: Point3, b: Point3) -> Self {
        AABB {
            x_interval: AABB::expand_zero_thickness(Vec2::new(a.0.min(b.0), a.0.max(b.0))),
            y_interval: AABB::expand_zero_thickness(Vec2::new(a.1.min(b.1), a.1.max(b.1))),
            z_interval: AABB::expand_zero_thickness(Vec2::new(a.2.min(b.2), a.2.max(b.2))),
        }
    }

    fn expand_zero_thickness(interval: Vec2) -> Vec2 {
        let delta = 0.00001;
        if interval.0 == interval.1 {
            Vec2::new(interval.0 - delta / 2.0, interval.1 + delta / 2.0)
        } else {
            interval
        }
    }

    pub fn from_aabb(a: &AABB, b: &AABB) -> Self {
        AABB {
            x_interval: Vec2::new(a[0].0.min(b[0].0), a[0].1.max(b[0].1)),
            y_interval: Vec2::new(a[1].0.min(b[1].0), a[1].1.max(b[1].1)),
            z_interval: Vec2::new(a[2].0.min(b[2].0), a[2].1.max(b[2].1)),
        }
    }

    pub fn hit(&self, ray: &Ray, t_range: Vec2) -> bool {
        let mut result = t_range;
        for idx in 0..3 {
            let interval = self[idx];
            let mut t0 = (interval.0 - ray.origin[idx]) / ray.direction[idx];
            let mut t1 = (interval.1 - ray.origin[idx]) / ray.direction[idx];
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

impl std::ops::Add<Vec3> for AABB {
    type Output = AABB;
    fn add(self, rhs: Vec3) -> Self::Output {
        AABB {
            x_interval: self.x_interval + rhs.0,
            y_interval: self.y_interval + rhs.1,
            z_interval: self.z_interval + rhs.2,
        }
    }
}
