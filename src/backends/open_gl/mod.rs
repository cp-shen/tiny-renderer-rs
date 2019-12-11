#![allow(non_snake_case)]

pub mod gl;

use crate::appdata::Model;
use crate::appdata::Scene;
use crate::appdata::Shader;
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

    let (ourShader, ourModel) = unsafe {
        // configure global opengl state
        // -----------------------------
        gl::Enable(gl::DEPTH_TEST);

        // build and compile shaders
        // -------------------------
        let ourShader = Shader::new("shaders/model_loading.vert", "shaders/model_loading.frag");

        // load models
        // -----------
        let ourModel = Model::new("resources/objects/nanosuit/nanosuit.obj");

        // draw in wireframe
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        (ourShader, ourModel)
    };

    unimplemented!();
}
