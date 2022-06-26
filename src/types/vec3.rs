use std::ops::{self, MulAssign};

use rand::random;

use crate::utils::random_range;

#[derive(Clone, Copy, Default)]
pub struct Vec3(pub f64, pub f64, pub f64);
pub type Point = Vec3;

impl Vec3 {
    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn square_magnitude(&self) -> f64 {
        self.0 * self.0
            + self.1 * self.1
            + self.2 * self.2
    }

    pub fn length(&self) -> f64 {
        self.square_magnitude().sqrt()
    }

    pub fn dot(a: Vec3, b: Vec3) -> f64 {
        a.0 * b.0 + a.1 * b.1 + a.2 * b.2
    }

    pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
        Vec3(u.1 * v.2 - u.2 * v.1,
            u.2 * v.0 - u.0 * v.2,
            u.0 * v.1 - u.1 * v.0)
    }

    pub fn normalize(u: &mut Vec3) {
        *u /= u.length()
    }

    pub fn normalized(u: Vec3) -> Vec3 {
        u / u.length()
    }

    pub fn random_vec() -> Vec3 {
        Vec3(random(), random(), random())
    }

    pub fn random_vec_bounded(min: f64, max: f64) -> Vec3 {
        Vec3(random_range(min, max), random_range(min, max), random_range(min, max))
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_vec_bounded(-1.0, 1.0);
            if p.length() < 1.0 {
                break p;
            }
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.mul_assign(1.0 / rhs);
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, 
            self.1 + rhs.1,
            self.2 + rhs.2)
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, 
            self.1 - rhs.1,
            self.2 - rhs.2)
    }
}

// element-wise product
impl ops::Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3(self.0 * rhs.0, 
            self.1 * rhs.1,
            self.2 * rhs.2)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3(self.0 * rhs, 
            self.1 * rhs,
            self.2 * rhs)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}