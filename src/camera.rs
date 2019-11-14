use cgmath::*;

type Mat4 = Matrix4<f32>;

#[derive(Clone, Copy)]
struct Camera {
    fov_y: f32,
    aspect: f32,
    near: f32,
    far: f32,
}

impl Camera {
    fn projection_mat() -> Mat4 {
        unimplemented!();
    }

    fn view_mat() -> Mat4 {
        unimplemented!();
    }
}
