use super::math::vector::{Vector3D};

struct Camera {
    eye: Vector3D,
    target: Vector3D,
    direction: Vector3D,
    right: Vector3D,
    up: Vector3D
}

impl Camera {
    pub fn new(eye: Option<Vector3D>, target: Option<Vector3D>) -> Self {
        let eye = eye.unwrap_or(Vector3D::new(0.0, 0.0, 0.0));
        let target = target.unwrap_or(Vector3D::new(0.0, 0.0, 0.0));
        let direction = eye.sub(&target).normalize();

        let up = Vector3D::new(0.0, 1.0, 0.0);
        let right = up.cross(&direction).normalize();
        let up = direction.cross(&right);

        Self {
            eye,
            target,
            direction,
            right,
            up
        }
    }
}
