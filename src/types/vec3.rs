use std::{
    ops::{
        self, 
        MulAssign
    }
};

use rand::random;

use crate::utils::{random_range};

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd)]
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

    pub fn random_unit_vector() -> Vec3 {
        Vec3::normalized(Vec3::random_in_unit_sphere())
    }

    pub fn random_in_hemisphere(normal: Vec3) -> Vec3{
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if Vec3::dot(in_unit_sphere, normal) > 0.0 {
            in_unit_sphere
        }
        else {
            -in_unit_sphere
        }
    }

    pub fn near_zero(&self) -> bool {
        const EPSILON: f64 = 1e-8;
        self.0.abs() < EPSILON && self.1.abs() < EPSILON && self.2.abs() < EPSILON
    }

    // reflect v across the plane defined by n
    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * Vec3::dot(v, n) * n
    }

    pub fn refract(uv: Vec3, n: Vec3, refractive_index_ratio: f64) -> Vec3 {
        let cos_theta = f64::min(1.0, Vec3::dot(-uv, n));
        let perpendicular = refractive_index_ratio * (uv + cos_theta * n);
        let x = (1.0 - perpendicular.square_magnitude()).abs().sqrt();
        let parallel = -x * n;
        perpendicular + parallel
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3(random_range(-1.0, 1.0), random_range(-1.0, 1.0), 0.0);
            if p.square_magnitude() <= 1.0 {
                break p;
            }
        }
    }

    pub fn lerp(a: Vec3, b: Vec3, t: f64) -> Vec3 {
        let clamped_t = t.clamp(0.0, 1.0);
        a * clamped_t + (1.0 - clamped_t) * b
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