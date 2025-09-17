use std::{cell::RefCell, f64::consts::PI};

use rand::{
    Rng,
    distr::{
        Distribution, StandardUniform,
        uniform::{SampleRange, SampleUniform},
    },
    rngs::ThreadRng,
};

use crate::vec::{Vec2, Vec3};

thread_local!(static RNG: RefCell<ThreadRng> = RefCell::new(rand::rng()));

pub fn m_random<T>() -> T
where
    StandardUniform: Distribution<T>,
{
    RNG.with(|rng| rng.borrow_mut().random::<T>())
}

pub fn m_random_range<T: SampleUniform, R: SampleRange<T>>(range: R) -> T {
    RNG.with(|rng| rng.borrow_mut().random_range(range))
}

pub fn random_vector_on_sphere(normal: Vec3) -> Vec3 {
    let p = Vec3::random_rage(-1.0..1.0).normalize();
    return if p.dot(normal) > 0.0 { p } else { -p };
}

pub fn random_in_disk() -> Vec2 {
    let radius = m_random::<f64>();
    let angle = m_random::<f64>() * PI * 2.0;
    Vec2::new(angle.cos() * radius, angle.sin() * radius)
}
