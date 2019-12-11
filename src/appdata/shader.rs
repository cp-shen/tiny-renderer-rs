#![allow(non_snake_case, dead_code)]

use crate::backends::open_gl::gl;
use crate::types::*;
use cgmath::prelude::*;
use gl::types::*;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Read;
use std::ptr;
use std::str;
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

            let fs = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fs, 1, &fs_code_c.as_ptr(), ptr::null());
            gl::CompileShader(fs);
            shader.checkCompileErrors(fs, "FRAGMENT");

            let pid = gl::CreateProgram();
            gl::AttachShader(pid, vs);
            gl::AttachShader(pid, fs);
            gl::LinkProgram(pid);
            shader.checkCompileErrors(pid, "PROGRAM");

            gl::DeleteShader(vs);
            gl::DeleteShader(fs);
            shader.id = pid;
        }

        shader
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    #[allow(dead_code, unused_variables)]
    unsafe fn checkCompileErrors(&self, shader: u32, t: &str) {
        let mut success = gl::FALSE as GLint;
        let mut infoLog: Vec<u8> = Vec::with_capacity(1024);
        infoLog.set_len(1024 - 1); // subtract 1 to skip the trailing null character

        if t != "PROGRAM" {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(
                    shader,
                    1024,
                    ptr::null_mut(),
                    infoLog.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::SHADER_COMPILATION_ERROR of type: {}\n{}\n \
                     -- --------------------------------------------------- -- ",
                    t,
                    str::from_utf8(&infoLog).unwrap()
                );
            }
        } else {
            gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(
                    shader,
                    1024,
                    ptr::null_mut(),
                    infoLog.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::PROGRAM_LINKING_ERROR of type: {}\n{}\n \
                     -- --------------------------------------------------- -- ",
                    t,
                    str::from_utf8(&infoLog).unwrap()
                );
            }
        }
    }

    pub unsafe fn useProgram(&self) {
        gl::UseProgram(self.id);
    }

    /// utility uniform functions
    /// ------------------------------------------------------------------------
    pub unsafe fn setBool(&self, name: &CStr, value: bool) {
        gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value as i32);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn setInt(&self, name: &CStr, value: i32) {
        gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn setFloat(&self, name: &CStr, value: f32) {
        gl::Uniform1f(gl::GetUniformLocation(self.id, name.as_ptr()), value);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn setVector3(&self, name: &CStr, value: &Vec3) {
        gl::Uniform3fv(
            gl::GetUniformLocation(self.id, name.as_ptr()),
            1,
            value.as_ptr(),
        );
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn setVec3(&self, name: &CStr, x: f32, y: f32, z: f32) {
        gl::Uniform3f(gl::GetUniformLocation(self.id, name.as_ptr()), x, y, z);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn setMat4(&self, name: &CStr, mat: &Mat4) {
        gl::UniformMatrix4fv(
            gl::GetUniformLocation(self.id, name.as_ptr()),
            1,
            gl::FALSE,
            mat.as_ptr(),
        );
    }
}
