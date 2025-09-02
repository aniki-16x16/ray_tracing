use core::panic;
use std::ops::{Index, Range};

use rand::distr::{Distribution, StandardUniform};
use serde::Deserialize;

use crate::{math::mix, random::m_random_range};

#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3(x, y, z)
    }
    pub fn from_single(v: f64) -> Self {
        Vec3(v, v, v)
    }
    pub fn from_axis_x(x: f64) -> Self {
        Vec3(x, 0.0, 0.0)
    }
    pub fn from_axis_y(y: f64) -> Self {
        Vec3(0.0, y, 0.0)
    }
    pub fn from_axis_z(z: f64) -> Self {
        Vec3(0.0, 0.0, z)
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
    pub fn cross(&self, v: Vec3) -> Self {
        Vec3(
            self.1 * v.2 - self.2 * v.1,
            self.2 * v.0 - self.0 * v.2,
            self.0 * v.1 - self.1 * v.0,
        )
    }
    pub fn sqrt(&self) -> Self {
        Vec3(self.0.sqrt(), self.1.sqrt(), self.2.sqrt())
    }
    pub fn floor(&self) -> Self {
        Vec3(self.0.floor(), self.1.floor(), self.2.floor())
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
    pub fn map(&self, f: impl Fn(f64) -> f64) -> Self {
        Vec3(f(self.0), f(self.1), f(self.2))
    }

    pub fn random_rage(range: Range<f64>) -> Self {
        Vec3(
            m_random_range(range.clone()),
            m_random_range(range.clone()),
            m_random_range(range),
        )
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Index out of bounds for Vec3"),
        }
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
impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
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

#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub struct Vec2(pub f64, pub f64);
impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Vec2(x, y)
    }
    pub fn from_single(v: f64) -> Self {
        Vec2(v, v)
    }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn normalize(&self) -> Self {
        let tmp = self.length();
        Vec2(self.0 / tmp, self.1 / tmp)
    }
    pub fn dot(&self, v: Vec3) -> f64 {
        self.0 * v.0 + self.1 * v.1
    }

    pub fn zero() -> Self {
        Vec2(0.0, 0.0)
    }
    pub fn one() -> Self {
        Vec2(1.0, 1.0)
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl std::ops::Add<f64> for Vec2 {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        Self(self.0 + rhs, self.1 + rhs)
    }
}
impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}
impl std::ops::AddAssign<f64> for Vec2 {
    fn add_assign(&mut self, rhs: f64) {
        self.0 += rhs;
        self.1 += rhs;
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl std::ops::Sub<f64> for Vec2 {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        Self(self.0 - rhs, self.1 - rhs)
    }
}

impl std::ops::Mul for Vec2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1)
    }
}
impl std::ops::Mul<f64> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}
impl std::ops::Mul<Vec2> for f64 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2(rhs.0 * self, rhs.1 * self)
    }
}

impl std::ops::Div for Vec2 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        if rhs.0 == 0.0 || rhs.1 == 0.0 {
            panic!("Cannot divide by zero!");
        }
        Self(self.0 / rhs.0, self.1 / rhs.1)
    }
}
impl std::ops::Div<f64> for Vec2 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        if rhs == 0.0 {
            panic!("Cannot divide by zero!");
        }
        Self(self.0 / rhs, self.1 / rhs)
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec2(-self.0, -self.1)
    }
}

impl Distribution<Vec2> for StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Vec2 {
        Vec2(rng.random(), rng.random())
    }
}

pub type Point2 = Vec2;
