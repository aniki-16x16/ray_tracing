use std::ops::Index;

use crate::{
    ray::Ray,
    vec::{Point3, Vec3},
};

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f64) -> Self {
        HitRecord { p, normal, t }
    }

    pub fn p(&self) -> &Point3 {
        &self.p
    }
    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }
    pub fn t(&self) -> f64 {
        self.t
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> Option<HitRecord>;
}

pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { list: vec![] }
    }

    pub fn push(&mut self, value: impl Hittable + 'static) {
        self.list.push(Box::new(value));
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Box<dyn Hittable>> {
        self.list.iter()
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> Option<HitRecord> {
        let mut result: Option<HitRecord> = None;
        let mut closest_so_far = t_range.1;
        for item in self.list.iter() {
            if let Some(record) = item.hit(ray, (t_range.0, closest_so_far)) {
                closest_so_far = record.t();
                result.replace(record);
            }
        }
        result
    }
}

impl Index<usize> for HittableList {
    type Output = Box<dyn Hittable>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.list[index]
    }
}
