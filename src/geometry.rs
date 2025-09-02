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
}

impl Hittable for GeometryEnum {
    fn hit<'a>(&'a self, ray: &Ray, t_range: Vec2) -> Option<HitRecord<'a>> {
        match self {
            GeometryEnum::Sphere(g) => g.hit(ray, t_range),
            GeometryEnum::Quad(g) => g.hit(ray, t_range),
        }
    }
    fn bounding_box(&self) -> &AABB {
        match self {
            GeometryEnum::Sphere(g) => g.bounding_box(),
            GeometryEnum::Quad(g) => g.bounding_box(),
        }
    }
}

pub struct Sphere<M: Material> {
    center: Point3,
    target_center: Point3,
    radius: f64,
    material: M,
    bbox: AABB,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Point3, target_center: Point3, radius: f64, material: M) -> Self {
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
                        material: &self.material,
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
    material: M,
    bbox: AABB,
    normal: Vec3,
    constant_d: f64,
    constant_w: Vec3,
}

impl<M: Material> Quad<M> {
    pub fn new(q: Vec3, u: Vec3, v: Vec3, material: M) -> Self {
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
                material: &self.material,
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
