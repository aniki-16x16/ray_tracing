use std::ops::Range;

use rand::distr::{Distribution, StandardUniform};

use crate::{math::mix, random::m_random_range};

#[derive(Debug, Clone, Copy)]
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

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn normalize(&self) -> Self {
        let tmp = self.length();
        Vec3(self.0 / tmp, self.1 / tmp, self.2 / tmp)
    }
    pub fn dot(&self, v: Vec3) -> f64 {
        self.0 * v.0 + self.1 * v.1 + self.2 * v.2
    }
    pub fn cross(&self, v: Vec3) -> Vec3 {
        Vec3(
            self.1 * v.2 - self.2 * v.1,
            self.2 * v.0 - self.0 * v.2,
            self.0 * v.1 - self.1 * v.0,
        )
    }
    pub fn sqrt(&self) -> Self {
        Vec3(self.0.sqrt(), self.1.sqrt(), self.2.sqrt())
    }

    pub fn zero() -> Self {
        Vec3(0.0, 0.0, 0.0)
    }
    pub fn one() -> Self {
        Vec3(1.0, 1.0, 1.0)
    }

    pub fn mix(a: Vec3, b: Vec3, t: f64) -> Self {
        Vec3(mix(a.0, b.0, t), mix(a.1, b.1, t), mix(a.2, b.2, t))
    }

    pub fn random_rage(range: Range<f64>) -> Self {
        Vec3(
            m_random_range(range.clone()),
            m_random_range(range.clone()),
            m_random_range(range),
        )
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
impl std::ops::Add<f64> for Vec3 {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        Self(self.0 + rhs, self.1 + rhs, self.2 + rhs)
    }
}
impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}
impl std::ops::AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        self.0 += rhs;
        self.1 += rhs;
        self.2 += rhs;
    }
}
impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}
impl std::ops::Sub<f64> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        Self(self.0 - rhs, self.1 - rhs, self.2 - rhs)
    }
}
impl std::ops::Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}
impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}
impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(rhs.0 * self, rhs.1 * self, rhs.2 * self)
    }
}
impl std::ops::Div for Vec3 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        if rhs.0 == 0.0 || rhs.1 == 0.0 || rhs.2 == 0.0 {
            panic!("Cannot divide by zero!");
        }
        Self(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}
impl std::ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        if rhs == 0.0 {
            panic!("Cannot divide by zero!");
        }
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}
impl std::ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl Distribution<Vec3> for StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        Vec3(rng.random(), rng.random(), rng.random())
    }
}

pub type Point3 = Vec3;
