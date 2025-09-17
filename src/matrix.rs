use crate::vec::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Mat33 {
    data: [[f64; 3]; 3],
}

impl Mat33 {
    pub fn new(data: [[f64; 3]; 3]) -> Self {
        Mat33 { data }
    }
}

impl std::ops::Mul<Vec3> for Mat33 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        let mut tmp = [0.0; 3];
        for row in 0..3 {
            for col in 0..3 {
                tmp[row] += self.data[row][col] * rhs[col];
            }
        }
        Vec3::new(tmp[0], tmp[1], tmp[2])
    }
}
