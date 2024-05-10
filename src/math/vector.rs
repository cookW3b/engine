trait Vector {
    fn magnitude(&self) -> f32;
    fn normalize(&self) -> Self;
    fn dot(&self, other: &Self) -> f32;
    fn cross(&self, other: &Self) -> Self;

    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn multiply(&self, scalar: f32) -> Self;
    fn divide(&self, scalar: f32) -> Self;
}

pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl Vector for Vector2D {
    fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    fn normalize(&self) -> Self {
        self.divide(self.magnitude())
    }

    fn dot(&self, other: &Self) -> f32 {
        todo!()
    }

    fn cross(&self, other: &Self) -> Self {
        todo!()
    }

    fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn multiply(&self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    fn divide(&self, scalar: f32) -> Self {
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
