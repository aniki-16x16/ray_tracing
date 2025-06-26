use crate::math::mix;

#[derive(Debug, Clone)]
pub struct Vec3(f64, f64, f64);

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3(x, y, z)
    }

    pub fn x(&self) -> f64 {
        self.0
    }
    pub fn y(&self) -> f64 {
        self.1
    }
    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn add(&self, v: &Vec3) -> Self {
        Vec3(self.0 + v.0, self.1 + v.1, self.2 + v.2)
    }
    pub fn add_n(&self, t: f64) -> Self {
        Vec3(self.0 + t, self.1 + t, self.2 + t)
    }
    pub fn subtract(&self, v: &Vec3) -> Self {
        Vec3(self.0 - v.0, self.1 - v.1, self.2 - v.2)
    }
    pub fn subtract_n(&self, t: f64) -> Self {
        Vec3(self.0 - t, self.1 - t, self.2 - t)
    }
    pub fn multiply(&self, t: f64) -> Self {
        Vec3(self.0 * t, self.1 * t, self.2 * t)
    }
    pub fn divide(&self, t: f64) -> Self {
        Vec3(self.0 / t, self.1 / t, self.2 / t)
    }

    pub fn length(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }
    pub fn normalize(&self) -> Self {
        let tmp = self.length();
        Vec3(self.0 / tmp, self.1 / tmp, self.2 / tmp)
    }
    pub fn dot(&self, v: &Vec3) -> f64 {
        self.0 * v.0 + self.1 * v.1 + self.2 * v.2
    }

    pub fn zero() -> Self {
        Vec3(0.0, 0.0, 0.0)
    }
    pub fn one() -> Self {
        Vec3(1.0, 1.0, 1.0)
    }

    pub fn mix(a: &Vec3, b: &Vec3, t: f64) -> Self {
        Vec3::new(mix(a.0, b.0, t), mix(a.1, b.1, t), mix(a.2, b.2, t))
    }
}

pub type Point3 = Vec3;
