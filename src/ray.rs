use crate::vec::{Point3, Vec3};

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: f64) -> Self {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}

impl std::ops::Add<Vec3> for Ray {
    type Output = Ray;
    fn add(self, rhs: Vec3) -> Self::Output {
        Ray {
            origin: self.origin + rhs,
            direction: self.direction,
            time: self.time,
        }
    }
}

impl std::ops::Sub<Vec3> for Ray {
    type Output = Ray;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Ray {
            origin: self.origin - rhs,
            direction: self.direction,
            time: self.time,
        }
    }
}
