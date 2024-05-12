use super::vector::{Vector2D, Vector3D, Vector4D};

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
            x: self.x.x * vector.x + self.y.x * vector.y,
            y: self.x.y * vector.x + self.y.y * vector.y,
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

#[derive(Default)]
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

    pub fn look_at(eye: Vector3D, target: Vector3D, up: Vector3D) -> Self {
        let direction = target.sub(&eye).normalize();
        let up_normal = up.normalize();
        let right = direction.cross(&up_normal).normalize();

        let up_look = right.cross(&direction);

        let result = Self::default();
    }
}
