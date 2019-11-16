use crate::*;
use cgmath::*;

#[derive(Clone, Copy)]
struct Camera {
    perspective: PerspectiveFov<f32>,
    position: Vec3,
    rotation: Quat,
}

impl Camera {
    fn projection_mat(&self) -> Mat4 {
        Mat4::from(self.perspective)
    }

    fn view_mat(&self) -> Mat4 {
        let rot_mat = Mat4::from(self.rotation);
        let trans_mat = Mat4::from_translation(self.position);
        trans_mat * rot_mat
    }
}
