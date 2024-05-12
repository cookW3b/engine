use super::vector;

pub struct Matrix2 {
    pub x: vector::Vector2D,
    pub y: vector::Vector2D,
}

impl Matrix2 {
    pub fn new(c0r0: f32, c0r1: f32, c1r0: f32, c1r1: f32) -> Self {
        Self {
            x: vector::Vector2D { x: c0r0, y: c0r1 },
            y: vector::Vector2D { x: c1r0, y: c1r1 },
        }
    }

    pub fn from_cols(c0: vector::Vector2D, c1: vector::Vector2D) -> Self {
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

    pub fn multiply_by_vector(&self, vector: &vector::Vector2D) -> vector::Vector2D {
        vector::Vector2D {
            x: self.x.x * vector.x + self.y.x * vector.y,
            y: self.x.y * vector.x + self.y.y * vector.y,
        }
    }

    pub fn multipy_by_matrix(&self, other: &Self) -> Self {
        todo!()
    }
}

pub struct Matrix3 {
    pub x: vector::Vector3D,
    pub y: vector::Vector3D,
    pub z: vector::Vector3D,
}

pub struct Matrix4 {
    pub x: vector::Vector4D,
    pub y: vector::Vector4D,
    pub z: vector::Vector4D,
    pub w: vector::Vector4D,
}
