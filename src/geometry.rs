use std::sync::Arc;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    material::{Material, MaterialEnum},
    math::get_sphere_uv,
    ray::Ray,
    vec::{Point3, Vec2, Vec3},
};

pub enum GeometryEnum {
    Sphere(Sphere<MaterialEnum>),
    Quad(Quad<MaterialEnum>),
    Cube(Cube<MaterialEnum>),
    Translate(Translate<GeometryEnum>),
}

impl Hittable for GeometryEnum {
    fn hit<'a>(&'a self, ray: &Ray, t_range: Vec2) -> Option<HitRecord<'a>> {
        match self {
            GeometryEnum::Sphere(g) => g.hit(ray, t_range),
            GeometryEnum::Quad(g) => g.hit(ray, t_range),
            GeometryEnum::Cube(g) => g.hit(ray, t_range),
            GeometryEnum::Translate(g) => g.hit(ray, t_range),
        }
    }
    fn bounding_box(&self) -> &AABB {
        match self {
            GeometryEnum::Sphere(g) => g.bounding_box(),
            GeometryEnum::Quad(g) => g.bounding_box(),
            GeometryEnum::Cube(g) => g.bounding_box(),
            GeometryEnum::Translate(g) => g.bounding_box(),
        }
    }
}

pub struct Sphere<M: Material> {
    center: Point3,
    target_center: Point3,
    radius: f64,
    material: Arc<M>,
    bbox: AABB,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Point3, target_center: Point3, radius: f64, material: Arc<M>) -> Self {
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

impl<M: Material + 'static> Hittable for Sphere<M> {
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

pub struct Quad<M: Material> {
    q: Vec3,
    edge: (Vec3, Vec3),
    material: Arc<M>,
    bbox: AABB,
    normal: Vec3,
    constant_d: f64,
    constant_w: Vec3,
}

impl<M: Material> Quad<M> {
    pub fn new(q: Vec3, u: Vec3, v: Vec3, material: Arc<M>) -> Self {
        let normal = u.cross(v);
        Quad {
            q,
            edge: (u, v),
            material,
            bbox: AABB::from_aabb(&AABB::new(q, q + u + v), &AABB::new(q + u, q + v)),
            normal,
            constant_d: normal.dot(q),
            constant_w: normal / normal.dot(normal),
        }
    }
}

impl<M: Material + 'static> Hittable for Quad<M> {
    fn hit<'a>(&'a self, ray: &Ray, t_range: Vec2) -> Option<HitRecord<'a>> {
        let (n, w) = (self.normal, self.constant_w);
        let denominator = n.dot(ray.direction);
        if denominator.abs() < 1e-8 {
            return None;
        }
        let t = (self.constant_d - n.dot(ray.origin)) / denominator;
        if t < t_range.0 || t > t_range.1 {
            return None;
        }
        let front_face = denominator < 0.0;
        let p = ray.at(t);
        let u_t = (p - self.q).cross(self.edge.1).dot(w);
        let v_t = (self.edge.0.cross(p - self.q)).dot(w);
        if u_t >= 0.0 && u_t <= 1.0 && v_t >= 0.0 && v_t <= 1.0 {
            Some(HitRecord {
                p,
                normal: (if front_face { n } else { -n }).normalize(),
                t,
                material: self.material.as_ref(),
                front_face,
                uv: Vec2::new(u_t, v_t),
            })
        } else {
            None
        }
    }
    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}

pub struct Cube<M: Material> {
    faces: [Quad<M>; 6],
    bbox: AABB,
}

impl<M: Material> Cube<M> {
    pub fn new(a: Vec3, b: Vec3, material: Arc<M>) -> Self {
        let min = Point3::new(a.0.min(b.0), a.1.min(b.1), a.2.min(b.2));
        let max = Point3::new(a.0.max(b.0), a.1.max(b.1), a.2.max(b.2));
        let dx = Vec3::from_axis_x(max.0 - min.0);
        let dy = Vec3::from_axis_y(max.1 - min.1);
        let dz = Vec3::from_axis_z(max.2 - min.2);
        Cube {
            faces: [
                Quad::new(min, dx, dz, material.clone()),       // bottom
                Quad::new(min + dy, dz, dx, material.clone()),  // top
                Quad::new(min + dz, dy, -dz, material.clone()), // left
                Quad::new(min + dx, dy, dz, material.clone()),  // right
                Quad::new(min + dz, dx, dy, material.clone()),  // front
                Quad::new(min, dy, dx, material.clone()),       // front
            ],
            bbox: AABB::new(a, b),
        }
    }
}

impl<M: Material + 'static> Hittable for Cube<M> {
    fn hit<'a>(&'a self, ray: &Ray, t_range: Vec2) -> Option<HitRecord<'a>> {
        let mut result: Option<HitRecord> = None;
        let mut closest_so_far = t_range.1;
        for item in self.faces.iter() {
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

pub struct Translate<G: Hittable> {
    instance: Box<G>,
    offset: Vec3,
    bbox: AABB,
}

impl<G: Hittable> Translate<G> {
    pub fn new(instance: G, offset: Vec3) -> Self {
        Translate {
            bbox: instance.bounding_box().clone() + offset,
            instance: Box::new(instance),
            offset,
        }
    }
}

impl<G: Hittable> Hittable for Translate<G> {
    fn hit<'a>(&'a self, ray: &Ray, t_range: Vec2) -> Option<HitRecord<'a>> {
        let ray = ray.clone() - self.offset;
        self.instance.hit(&ray, t_range)
    }
    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
