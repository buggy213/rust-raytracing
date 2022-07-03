use std::{ops::{Add, Mul}, convert::identity};

use super::vec3::Vec3;

// Very inefficient code for handling transformation matrices

/// https://en.wikipedia.org/wiki/Transformation_matrix#Affine_transformations
/// https://en.wikipedia.org/wiki/Quaternions_and_spatial_rotation#Quaternion-derived_rotation_matrix
/// https://en.wikipedia.org/wiki/Quaternion

#[derive(Clone, Copy)]
pub struct Transform {
    data: [[f64; 4]; 4]
}

#[derive(Clone, Copy)]
pub struct Quaternion {
    a: f64,
    b: f64,
    c: f64,
    d: f64
}

impl Quaternion {
    pub fn get_transform(&self) -> Transform {
        let mut matrix = [[0.0; 4]; 4];
        let s = 1.0 / (self.a * self.a + self.b * self.b + self.c * self.c + self.d * self.d);
        matrix[0][0] = 1.0 - 2.0 * s * (self.c * self.c + self.d * self.d);
        matrix[0][1] = 2.0 * s * (self.b * self.c - self.d * self.a);
        matrix[0][2] = 2.0 * s * (self.b * self.d + self.a * self.c);

        matrix[1][0] = 2.0 * s * (self.b * self.c + self.d * self.a);
        matrix[1][1] = 1.0 - 2.0 * s * (self.b * self.b + self.d * self.d);
        matrix[1][2] = 2.0 * s * (self.c * self.d - self.b * self.a);

        matrix[2][0] = 2.0 * s * (self.b * self.d - self.c * self.a);
        matrix[2][1] = 2.0 * s * (self.c * self.d + self.a * self.b);
        matrix[2][2] = 1.0 - 2.0 * s * (self. b * self.b + self.c * self.c);

        matrix[3][3] = 1.0;

        Transform { data: matrix }
    }
}

impl Transform {
    pub fn identity() -> Transform {
        let mut data = [[0.0; 4]; 4];
        data[0][0] = 1.0;
        data[1][1] = 1.0;
        data[2][2] = 1.0;
        data[3][3] = 1.0;
        Transform { data }
    }

    pub fn translate(&self, translation: Vec3) -> Transform {
        let mut eye = Transform::identity();
        eye.data[0][3] = translation.0;
        eye.data[1][3] = translation.1;
        eye.data[2][3] = translation.2;
        eye
    }

    pub fn rotate_euler(&self, rotation: Vec3) -> Transform {
        self.rotate_angle_axis(Vec3(1.0, 0.0, 0.0), rotation.0)
            .rotate_angle_axis(Vec3(0.0, 1.0, 0.0), rotation.1)
            .rotate_angle_axis(Vec3(0.0, 0.0, 1.0), rotation.2)
    }

    pub fn rotate_quaternion(&self, rotation: Quaternion) -> Transform {
        let rotation = rotation.get_transform();
        self.compose(rotation)
    }

    pub fn rotate_angle_axis(&self, axis: Vec3, angle: f64) -> Transform {
        let axis = Vec3::normalized(axis);
        let a = (angle / 2.0).cos();
        let x = (angle / 2.0).sin();
        let b = axis.x() * x;
        let c = axis.y() * x;
        let d = axis.z() * x;
        let q = Quaternion { a, b, c, d };
        self.rotate_quaternion(q)
    }

    pub fn compose(&self, other: Transform) -> Transform {
        // OTHER matmul SELF
        let mut data = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                data[i][j] = Transform::dot(*self, i, other, j);
            }
        }

        Transform { data }
    }

    fn dot(a: Transform, row: usize, b: Transform, col: usize) -> f64 {
        a.data[row][0] * b.data[0][col] + 
        a.data[row][1] * b.data[1][col] + 
        a.data[row][2] * b.data[2][col] + 
        a.data[row][3] * b.data[3][col]
    }
}
