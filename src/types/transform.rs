use super::{vec3::Vec3, ray::Ray};

// Very inefficient code for handling transformation matrices

/// https://en.wikipedia.org/wiki/Transformation_matrix#Affine_transformations
/// https://en.wikipedia.org/wiki/Quaternions_and_spatial_rotation#Quaternion-derived_rotation_matrix
/// https://en.wikipedia.org/wiki/Quaternion

#[derive(Clone, Copy)]
pub struct TransformData {
    data: Matrix4x4,
    inverse: Matrix4x4
}

#[derive(Clone, Copy)]
pub struct Quaternion {
    a: f64,
    b: f64,
    c: f64,
    d: f64
}

impl Quaternion {
    pub fn get_transform(&self) -> TransformData {
        TransformData { data: self.get_transformation_matrix(), inverse: self.inv().get_transformation_matrix() }
    }

    fn inv(&self) -> Quaternion {
        let square_magnitude = self.square_magnitude();
        let a = self.a / square_magnitude;
        let b = -self.b / square_magnitude;
        let c = -self.c / square_magnitude;
        let d = -self.d / square_magnitude;
        Quaternion { a, b, c, d }
    }

    fn square_magnitude(&self) -> f64 {
        self.a * self.a + self.b * self.b + self.c * self.c + self.d * self.d
    }

    fn get_transformation_matrix(&self) -> Matrix4x4 {
        let mut matrix = [[0.0; 4]; 4];
        let s = 1.0 / self.square_magnitude();
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
        matrix
    }
}
#[derive(Clone, Copy)]
struct Vec4(f64, f64, f64, f64);

type Matrix4x4 = [[f64; 4]; 4];

impl TransformData {
    pub fn identity() -> TransformData {
        let mut data = [[0.0; 4]; 4];
        data[0][0] = 1.0;
        data[1][1] = 1.0;
        data[2][2] = 1.0;
        data[3][3] = 1.0;
        TransformData { data: data.clone(), inverse: data }
    }

    pub fn translate(&self, translation: Vec3) -> TransformData {
        let mut eye = TransformData::identity();
        eye.data[0][3] = translation.0;
        eye.data[1][3] = translation.1;
        eye.data[2][3] = translation.2;
        eye.inverse[0][3] = -translation.0;
        eye.inverse[1][3] = -translation.1;
        eye.inverse[2][3] = -translation.2;
        eye
    }

    pub fn rotate_euler(&self, rotation: Vec3) -> TransformData {
        self.rotate_angle_axis(Vec3(1.0, 0.0, 0.0), rotation.0)
            .rotate_angle_axis(Vec3(0.0, 1.0, 0.0), rotation.1)
            .rotate_angle_axis(Vec3(0.0, 0.0, 1.0), rotation.2)
    }

    pub fn rotate_quaternion(&self, rotation: Quaternion) -> TransformData {
        let rotation = rotation.get_transform();
        self.compose(rotation)
    }

    pub fn rotate_angle_axis(&self, axis: Vec3, angle: f64) -> TransformData {
        let axis = Vec3::normalized(axis);
        let a = (angle / 2.0).cos();
        let x = (angle / 2.0).sin();
        let b = axis.x() * x;
        let c = axis.y() * x;
        let d = axis.z() * x;
        let q = Quaternion { a, b, c, d };
        self.rotate_quaternion(q)
    }

    pub fn compose(&self, other: TransformData) -> TransformData {
        // OTHER matmul SELF for forward direction
        // SELF.INVERSE matmul OTHER for inverse
        TransformData { 
            data: TransformData::matmul(other.data, self.data),
            inverse: TransformData::matmul(self.inverse, other.inverse)
        }
    }

    pub fn matmul(a: Matrix4x4, b: Matrix4x4) -> Matrix4x4 {
        let mut data = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                data[i][j] = TransformData::dot(a, i, b, j);
            }
        }
        data
    }

    fn apply_to_vec4(matrix: Matrix4x4, vector: Vec4) -> Vec4 {
        Vec4(
            TransformData::dot_vec4(matrix, 0, vector),
            TransformData::dot_vec4(matrix, 1, vector),
            TransformData::dot_vec4(matrix, 2, vector),
            TransformData::dot_vec4(matrix, 3, vector),
        )
    }

    fn dot_vec4(a: Matrix4x4, row: usize, vector: Vec4) -> f64 {
        a[row][0] * vector.0 +
        a[row][1] * vector.1 +
        a[row][2] * vector.2 +
        a[row][3] * vector.3
    }

    fn dot(a: Matrix4x4, row: usize, b: Matrix4x4, col: usize) -> f64 {
        a[row][0] * b[0][col] + 
        a[row][1] * b[1][col] + 
        a[row][2] * b[2][col] + 
        a[row][3] * b[3][col]
    }
}


pub trait Transform<T> {
    fn transform(&self, transform: TransformData) -> T;
}

pub trait InverseTransform<T> {
    fn inverse_transform(&self, transform: TransformData) -> T;
}

impl Transform<Vec3> for Vec3 {
    fn transform(&self, transform: TransformData) -> Vec3 {
        let extended_vec = Vec4(self.0, self.1, self.2, 1.0);
        let transformed = TransformData::apply_to_vec4(transform.data, extended_vec);
        Vec3(
            transformed.0,
            transformed.1,
            transformed.2
        )
    }
}

impl InverseTransform<Vec3> for Vec3 {
    fn inverse_transform(&self, transform: TransformData) -> Vec3 {
        let extended_vec = Vec4(self.0, self.1, self.2, 1.0);
        let transformed = TransformData::apply_to_vec4(transform.inverse, extended_vec);
        Vec3(
            transformed.0,
            transformed.1,
            transformed.2
        )
    }
}

impl Transform<Ray> for Ray {
    fn transform(&self, transform: TransformData) -> Ray {
        Ray { origin: self.origin.transform(transform), direction: self.direction.transform(transform), time: self.time }
    }
}

impl InverseTransform<Ray> for Ray {
    fn inverse_transform(&self, transform: TransformData) -> Ray {
        Ray { origin: self.origin.inverse_transform(transform), direction: self.direction.inverse_transform(transform), time: self.time }
    }
}