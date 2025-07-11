use std::{cell::RefCell, f64::consts::PI};

use rand::{
    Rng,
    distr::{
        Distribution, StandardUniform,
        uniform::{SampleRange, SampleUniform},
    },
    rngs::ThreadRng,
};

use crate::vec::Vec3;

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
    loop {
        let p = Vec3::random_rage(-1.0..1.0);
        let length_sq = p.length_squared();
        if length_sq > 1e-160 {
            let result = p / length_sq;
            return if result.dot(normal) > 0.0 {
                result
            } else {
                -result
            };
        }
    }
}

pub fn random_in_disk() -> (f64, f64) {
    let radius = m_random::<f64>();
    let angle = m_random::<f64>() * PI * 2.0;
    (angle.cos() * radius, angle.sin() * radius)
}
