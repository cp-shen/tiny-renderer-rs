use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Read;
use std::ptr;

pub struct Shader {
    id: u32,
}

impl Shader {
    pub fn new(vs_path: &str, fs_path: &str) -> Shader {
        let mut shader = Shader { id: 0 };

        let mut vs_file =
            File::open(vs_path).unwrap_or_else(|_| panic!("Failed to open {}", vs_path));
        let mut fs_file =
            File::open(fs_path).unwrap_or_else(|_| panic!("Failed to open {}", vs_path));

        let mut vs_code = String::new();
        let mut fs_code = String::new();

        vs_file
            .read_to_string(&mut vs_code)
            .expect("Failed to read vertex shader");
        fs_file
            .read_to_string(&mut fs_code)
            .expect("Failed to read vertex shader");

        let vs_code_c = CString::new(vs_code.as_bytes()).unwrap();
        let fs_code_c = CString::new(fs_code.as_bytes()).unwrap();

        // compile shaderss
        unsafe {
            let vs = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vs, 1, &vs_code_c.as_ptr(), ptr::null());
            gl::CompileShader(vs);
            shader.checkCompileErrors(vs, "VERTEX");
        }

        shader
    }

    #[allow(dead_code, non_snake_case)]
    unsafe fn checkCompileErrors(&self, shader: u32, t: &str) {
        unimplemented!();
    }
}
