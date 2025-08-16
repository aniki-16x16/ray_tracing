use crate::{
    math::mix,
    random::{m_random, m_random_range},
    vec::{Point3, Vec3},
};

pub struct PerlinNoise {
    rand_value: [f64; 256],
    perms: ([usize; 256], [usize; 256], [usize; 256]),
}

impl PerlinNoise {
    pub fn new() -> Self {
        PerlinNoise {
            rand_value: [0.0; 256].map(|_| m_random()),
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
        let grid = p.floor();
        let offset = p - grid;
        let x0 = mix(
            self.value_at(grid),
            self.value_at(grid + Vec3::new(1.0, 0.0, 0.0)),
            offset.0,
        );
        let x1 = mix(
            self.value_at(grid + Vec3::new(0.0, 1.0, 0.0)),
            self.value_at(grid + Vec3::new(1.0, 1.0, 0.0)),
            offset.0,
        );
        let y0 = mix(x0, x1, offset.1);
        let x2 = mix(
            self.value_at(grid + Vec3::new(0.0, 0.0, 1.0)),
            self.value_at(grid + Vec3::new(1.0, 0.0, 1.0)),
            offset.0,
        );
        let x3 = mix(
            self.value_at(grid + Vec3::new(0.0, 1.0, 1.0)),
            self.value_at(grid + Vec3::new(1.0, 1.0, 1.0)),
            offset.0,
        );
        let y1 = mix(x2, x3, offset.1);
        mix(y0, y1, offset.2)
    }

    fn value_at(&self, p: Point3) -> f64 {
        let i = (p.0 as i32 & 255) as usize;
        let j = (p.1 as i32 & 255) as usize;
        let k = (p.2 as i32 & 255) as usize;
        self.rand_value[self.perms.0[i] ^ self.perms.1[j] ^ self.perms.2[k]]
    }
}
