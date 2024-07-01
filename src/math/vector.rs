use std::ops::{Index, IndexMut, Add, Div, Sub, Mul};

use super::matrix::Matrix4;

pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl Vector2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        self.divide(self.magnitude())
    }

    pub fn dot(&self, other: &Self) -> f32 {
        todo!()
    }

    pub fn cross(&self, other: &Self) -> Self {
        todo!()
    }

    pub fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn multiply(&self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    pub fn divide(&self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        self.divide(self.magnitude())
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn multiply(&self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    pub fn divide(&self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct Vector4D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4D {
    pub fn multiple_by_scalar(&self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }

    pub fn add_vector(&self, other: &Vector4D) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w
        }
    }
}

impl Index<usize> for Vector4D {
    type Output = f32;

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

impl IndexMut<usize> for Vector4D {
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

impl Add<f32> for Vector4D {
    type Output = Self;

    fn add(self, v: f32) -> Self {
        Self {
            x: self.x + v,
            y: self.y + v,
            z: self.z + v,
            w: self.w + v
        }
    }
}

impl Add<Vector4D> for Vector4D {
    type Output = Self;

    fn add(self, other: Vector4D) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w
        }
    }
}

impl Sub<f32> for Vector4D {
    type Output = Self;

    fn sub(self, v: f32) -> Self {
        Self {
            x: self.x - v,
            y: self.y - v,
            z: self.z - v,
            w: self.w - v
        }
    }
}

impl Sub<Vector4D> for Vector4D {
    type Output = Self;

    fn sub(self, other: Vector4D) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w
        }
    }
}

impl Mul<f32> for Vector4D {
    type Output = Self;

    fn mul(self, v: f32) -> Self {
        Self {
            x: self.x * v,
            y: self.y * v,
            z: self.z * v,
            w: self.w * v
        }
    }
}

impl Mul<Vector4D> for Vector4D {
    type Output = f32;

    fn mul(self, other: Vector4D) -> f32 {
        let mut sum = 0.0;

        for i in 0..4 {
            sum += self[i] * other[i]
        }

        sum
    }
}

impl Mul<Matrix4> for Vector4D {
    type Output = Self;

    fn mul(self, matrix: Matrix4) -> Vector4D {
        let mut result = Vector4D::default();
        for i in 0..4 {
            result[i] = self * matrix[i];
        }
        result
    }
}

impl Div<f32> for Vector4D {
    type Output = Self;

    fn div(self, v: f32) -> Self {
        Self {
            x: self.x / v,
            y: self.y / v,
            z: self.z / v,
            w: self.w / v
        }
    }
}
