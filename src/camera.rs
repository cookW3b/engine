use super::math::vector::Vector3D;
use super::math::matrix::Matrix4;

pub struct Camera {
    pub eye: Vector3D,
    pub target: Vector3D,
    pub up: Vector3D,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> Matrix4 {
        let view = Matrix4::look_at(&self.eye, &self.target, &self.up);
        let proj = Matrix4::perspective(self.fovy, self.aspect, self.znear, self.zfar);
        // let OPENGL_TO_WGPU_MATRIX = Matrix4::new(
        //     1.0, 0.0, 0.0, 0.0,
        //     0.0, 1.0, 0.0, 0.0,
        //     0.0, 0.0, 0.5, 0.5,
        //     0.0, 0.0, 0.0, 1.0,
        // );
        // OPENGL_TO_WGPU_MATRIX.multiply(&proj.multiply(&view))
        // println!("proj: {:#?}", proj.to_array());
        // proj.multiply(&view)
        proj * view
    }
}
