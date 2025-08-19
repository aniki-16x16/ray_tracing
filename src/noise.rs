use crate::{
    math::hermite_t,
    random::m_random_range,
    vec::{Point3, Vec3},
};

pub struct PerlinNoise {
    rand_value: [Vec3; 256],
    perms: ([usize; 256], [usize; 256], [usize; 256]),
}

impl PerlinNoise {
    pub fn new() -> Self {
        PerlinNoise {
            rand_value: std::array::from_fn(|_| Vec3::random_rage(-1.0..1.0)),
            perms: (
                Self::permute(std::array::from_fn(|i| i)),
                Self::permute(std::array::from_fn(|i| i)),
                Self::permute(std::array::from_fn(|i| i)),
            ),
        }
    }

    fn permute(mut value: [usize; 256]) -> [usize; 256] {
        for idx in 0..256 {
            let target = m_random_range(0..256);
            let (a, b) = (value[idx], value[target]);
            value[idx] = b;
            value[target] = a;
        }
        value
    }

    pub fn value(&self, p: Point3) -> f64 {
        let floor_p = p.floor();
        let fract_p = p - floor_p;
        let smooth = fract_p.map(hermite_t);
        let mut result = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let offset = Vec3::new(i as f64, j as f64, k as f64);
                    result += ((1.0 - i as f64) * (1.0 - smooth.0) + i as f64 * smooth.0)
                        * ((1.0 - j as f64) * (1.0 - smooth.1) + j as f64 * smooth.1)
                        * ((1.0 - k as f64) * (1.0 - smooth.2) + k as f64 * smooth.2)
                        * self.value_at(floor_p + offset).dot(fract_p - offset);
                }
            }
        }
        result
    }

    fn value_at(&self, p: Point3) -> Vec3 {
        let i = (p.0 as i32 & 255) as usize;
        let j = (p.1 as i32 & 255) as usize;
        let k = (p.2 as i32 & 255) as usize;
        self.rand_value[self.perms.0[i] ^ self.perms.1[j] ^ self.perms.2[k]]
    }
}
