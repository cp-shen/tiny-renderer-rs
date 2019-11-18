use crate::*;
use cgmath::*;

#[allow(dead_code)]
pub struct Scene {
    models: Vec<tobj::Model>,
    camera: Camera,
}

impl Scene {
    #[allow(dead_code)]
    fn new() -> Scene {
        Scene {
            models: vec![],
            camera: Camera::new(),
        }
    }
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct Camera {
    perspective: PerspectiveFov<f32>,
    position: Vec3,
    rotation: Quat,
}

#[allow(dead_code)]
impl Camera {
    fn projection_mat(&self) -> Mat4 {
        Mat4::from(self.perspective)
    }

    fn view_mat(&self) -> Mat4 {
        let rot_mat = Mat4::from(self.rotation);
        let trans_mat = Mat4::from_translation(self.position);
        trans_mat * rot_mat
    }

    fn new() -> Camera {
        Camera {
            perspective: PerspectiveFov {
                fovy: Rad::from(Deg(45_f32)),
                aspect: 4_f32 / 3_f32,
                near: 0.1,
                far: 100_f32,
            },
            position: Vec3::new(0_f32, 0_f32, 0_f32),
            rotation: Quat::new(1_f32, 0_f32, 0_f32, 0_f32),
        }
    }
}
