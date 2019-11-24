use crate::types::*;
use cgmath::*;

pub struct DirectionalLight {
    dir: Vec3,
}

impl DirectionalLight {
    pub fn new() -> DirectionalLight {
        DirectionalLight {
            dir: Vec3::new(0_f32, 0_f32, 1_f32),
        }
    }

    #[allow(dead_code)]
    fn set_dir(&mut self, dir: Vec3) {
        if dir.magnitude() == 0_f32 {
            panic!()
        }
        self.dir = dir;
    }
}
