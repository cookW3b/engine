use super::vector::{Vector2D, Vector3D, Vector4D};
use std::{
    f32::consts::PI,
    ops::{Index, IndexMut, Mul},
};

pub struct Matrix2 {
    pub x: Vector2D,
    pub y: Vector2D,
}

impl Matrix2 {
    pub fn new(c0r0: f32, c0r1: f32, c1r0: f32, c1r1: f32) -> Self {
        Self {
            x: Vector2D { x: c0r0, y: c0r1 },
            y: Vector2D { x: c1r0, y: c1r1 },
        }
    }

    pub fn from_cols(c0: Vector2D, c1: Vector2D) -> Self {
        Self { x: c0, y: c1 }
    }

    pub fn add(&self, other: &Self) -> Self {
        Self::from_cols(self.x.add(&other.x), self.y.add(&other.y))
    }

    pub fn sub(&self, other: &Self) -> Self {
        Self::from_cols(self.x.sub(&other.x), self.y.sub(&other.y))
    }

    pub fn multiply_by_scalar(&self, scalar: f32) -> Self {
        Self::from_cols(self.x.multiply(scalar), self.y.multiply(scalar))
    }

    pub fn multiply_by_vector(&self, vector: &Vector2D) -> Vector2D {
        Vector2D {
            x: self.x.x * vector.x + self.x.y * vector.y,
            y: self.y.x * vector.x + self.y.y * vector.y,
        }
    }

    pub fn multipy_by_matrix(&self, other: &Self) -> Self {
        todo!()
    }
}

pub struct Matrix3 {
    pub x: Vector3D,
    pub y: Vector3D,
    pub z: Vector3D,
}

#[derive(Default, Clone)]
pub struct Matrix4 {
    pub x: Vector4D,
    pub y: Vector4D,
    pub z: Vector4D,
    pub w: Vector4D,
}

impl Matrix4 {
    #[rustfmt::skip]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        c0r0: f32, c0r1: f32, c0r2: f32, c0r3: f32,
        c1r0: f32, c1r1: f32, c1r2: f32, c1r3: f32,
        c2r0: f32, c2r1: f32, c2r2: f32, c2r3: f32,
        c3r0: f32, c3r1: f32, c3r2: f32, c3r3: f32
    ) -> Self {
        Self {
            x: Vector4D { x: c0r0, y: c0r1, z: c0r2, w: c0r3 },
            y: Vector4D { x: c1r0, y: c1r1, z: c1r2, w: c1r3 },
            z: Vector4D { x: c2r0, y: c2r1, z: c2r2, w: c2r3 },
            w: Vector4D { x: c3r0, y: c3r1, z: c3r2, w: c3r3 },
        }
    }

    pub fn from_cols(c0: Vector4D, c1: Vector4D, c2: Vector4D, c3: Vector4D) -> Self {
        Self {
            x: c0,
            y: c1,
            z: c2,
            w: c3,
        }
    }

    pub fn to_array(&self) -> [[f32; 4]; 4] {
        [
            [self.x.x, self.x.y, self.x.z, self.x.w],
            [self.y.x, self.y.y, self.y.z, self.y.w],
            [self.z.x, self.z.y, self.z.z, self.z.w],
            [self.w.x, self.w.y, self.w.z, self.w.w],
        ]
    }

    pub fn look_at(eye: &Vector3D, target: &Vector3D, up: &Vector3D) -> Self {
        let f = target.sub(eye).normalize();
        let s = f.cross(up).normalize();
        let u = s.cross(&f);

        let mut result = Matrix4::default();

        result[0][0] = s.x;
        result[0][1] = u.x;
        result[0][2] = -f.x;
        result[1][0] = s.y;
        result[1][1] = u.y;
        result[1][2] = -f.y;
        result[2][0] = s.z;
        result[2][1] = u.z;
        result[2][2] = -f.z;
        result[3][0] = -s.dot(eye);
        result[3][1] = -u.dot(eye);
        result[3][2] = f.dot(eye);
        result[3][3] = 1.0;

        result
    }

    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        let fov_rad = fov * (PI / 180.0);
        let mut perspective_matrix = Self::default();
        let distance = (fov_rad * 0.5).tan().recip();

        perspective_matrix[0][0] = distance / aspect;
        perspective_matrix[1][1] = distance;
        perspective_matrix[2][2] = (far + near) / (near - far);
        perspective_matrix[2][3] = -1.0;
        perspective_matrix[3][2] = (2.0 * far * near) / (near - far);

        perspective_matrix
    }
}

impl Index<usize> for Matrix4 {
    type Output = Vector4D;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Matrix4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl Mul<Matrix4> for Matrix4 {
    type Output = Self;

    fn mul(self, other: Matrix4) -> Self {
        let mut result = Matrix4::default();

        result
    }
}
