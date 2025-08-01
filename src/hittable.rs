use std::{ops::Index, sync::Arc};

use crate::{
    aabb::AABB,
    material::Material,
    ray::Ray,
    vec::{Point3, Vec2, Vec3},
};

pub struct HitRecord<'mat> {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub material: &'mat dyn Material,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        p: Point3,
        normal: Vec3,
        t: f64,
        material: &'a dyn Material,
        front_face: bool,
    ) -> Self {
        HitRecord {
            p,
            normal,
            t,
            material: material,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit<'a>(&'a self, ray: &Ray, t_range: Vec2) -> Option<HitRecord<'a>>;
    fn bounding_box(&self) -> &AABB;
}

pub struct HittableList {
    pub list: Vec<Arc<dyn Hittable>>,
    bbox: AABB,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            list: vec![],
            bbox: AABB::default(),
        }
    }

    pub fn push(&mut self, value: impl Hittable + 'static) -> &mut Self {
        self.bbox = AABB::from_aabb(&self.bbox, value.bounding_box());
        self.list.push(Arc::new(value));
        self
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Arc<dyn Hittable>> {
        self.list.iter()
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_range: Vec2) -> Option<HitRecord> {
        let mut result: Option<HitRecord> = None;
        let mut closest_so_far = t_range.1;
        for item in self.list.iter() {
            if let Some(record) = item.hit(ray, Vec2::new(t_range.0, closest_so_far)) {
                closest_so_far = record.t;
                result.replace(record);
            }
        }
        result
    }
    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}

impl Index<usize> for HittableList {
    type Output = Arc<dyn Hittable>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.list[index]
    }
}
