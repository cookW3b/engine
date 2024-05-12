pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl Vector2D {
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
    x: f32,
    y: f32,
    z: f32,
}

pub struct Vector4D {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}
