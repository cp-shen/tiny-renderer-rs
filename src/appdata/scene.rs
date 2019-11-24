use crate::appdata::camera::*;
use crate::appdata::light::*;
use std::path::Path;

#[allow(dead_code)]
pub struct Scene {
    pub models: Vec<tobj::Model>,
    pub camera: Camera,
    pub directional_light: DirectionalLight,
}

impl Scene {
    #[allow(dead_code)]
    pub fn new() -> Scene {
        Scene {
            models: vec![],
            camera: Camera::new(),
            directional_light: DirectionalLight::new(),
        }
    }

    #[allow(dead_code)]
    pub fn load_obj(file_name: &Path) -> Scene {
        let mod_load_result = tobj::load_obj(file_name);
        assert!(mod_load_result.is_ok());

        let (models, _) = mod_load_result.unwrap();

        Scene {
            models,
            camera: Camera::new(),
            directional_light: DirectionalLight::new(),
        }
    }
}
