use super::vec3::{Vec3, Point};

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
    pub time: f64
}

impl Ray {
    pub fn at(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }
}