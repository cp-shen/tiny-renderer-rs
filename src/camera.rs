use cgmath::*;
use crate::*;

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
        unimplemented!();
    }
}
