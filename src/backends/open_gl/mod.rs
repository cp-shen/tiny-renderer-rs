pub mod gl;

use crate::appdata::scene::Scene;
use glfw::Context;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

#[allow(unused_variables)]
pub fn render_with_opengl(scene: &Scene) {
    // init glfw
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    // create window
    let (mut window, events) = glfw
        .create_window(
            SCR_WIDTH,
            SCR_HEIGHT,
            std::env::current_exe().unwrap().to_str().unwrap(),
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create glfw window");
    window.make_current();
    window.set_framebuffer_size_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);

    // load OpenGL functions
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    unimplemented!();
}
