use crate::appdata::Scene;
use crate::backends::*;

pub fn render(scene: &Scene, backend: Backend) {
    match backend {
        Backend::OpenGL => open_gl::render_with_opengl(scene),
        Backend::MyGL => my_gl::render_with_mygl(scene),
    }
}
