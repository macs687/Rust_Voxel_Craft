use std::error::Error;
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::ptr;
use bytemuck::bytes_of;
use gl::types::*;
use glam::{Mat4};
use anyhow::{Context, Result};


pub struct Shader{
    id: GLuint,
}


impl Shader {
    pub fn new(id: GLuint) -> Self {
        Self { id }
    }

    pub fn use_shader(&self) {
        unsafe { gl::UseProgram(self.id); }
    }

    pub fn uniform_matrix(&self, name: &str, matrix: Mat4){
        unsafe {
            let c_name = CString::new(name).expect("CString::new failed");
            let transform_loc = gl::GetUniformLocation(self.id, c_name.as_ptr());
            gl::UniformMatrix4fv(transform_loc, 1, gl::FALSE, matrix.as_ref().as_ptr());
        }
    }
}


impl Drop for Shader {
    fn drop(&mut self){
        unsafe { gl::DeleteProgram(self.id) }
    }
}


pub fn load_shader(vertex_file: &str, fragment_file: &str) -> Result<Shader, Box<dyn Error>> {
    let mut vertex_code = String::new();
    let mut fragment_code = String::new();

    let mut v_shader_file = File::open(vertex_file).map_err(|e| {
        format!("Failed to open vertex shader file. File not found in '{}': {}", vertex_file, e)
    })?;

    let mut f_shader_file = File::open(fragment_file).map_err(|e| {
        format!("Failed to open fragment shader file. File not found in '{}': {}", fragment_file, e)
    })?;

    v_shader_file.read_to_string(&mut vertex_code).map_err(|e| {
        format!("Failed to read vertex shader file '{}': {}", vertex_file, e)
    })?;

    f_shader_file.read_to_string(&mut fragment_code).map_err(|e| {
        format!("Failed to read fragment shader file '{}': {}", fragment_file, e)
    })?;

    // Преобразование кода шейдеров в CString
    let v_shader_code = CString::new(vertex_code).map_err(|e| {
        format!("Failed to convert vertex shader code to CString: {}", e)
    })?;

    let f_shader_code = CString::new(fragment_code).map_err(|e| {
        format!("Failed to convert fragment shader code to CString: {}", e)
    })?;

    let mut success: GLint = 0;
    let mut info_log: [GLchar; 512] = [0; 512];

    let vertex = unsafe {
        let vertex = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(vertex, 1, &v_shader_code.as_ptr(), ptr::null());
        gl::CompileShader(vertex);
        gl::GetShaderiv(vertex, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            gl::GetShaderInfoLog(vertex, 512, ptr::null_mut(), info_log.as_mut_ptr());
            return Err(
                format!(
                    "SHADER::VERTEX: compilation failed: {}",
                    std::str::from_utf8(bytes_of(&info_log))?
                ).into()
            );
        }
        vertex
    };

    let fragment = unsafe {
        let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(fragment, 1, &f_shader_code.as_ptr(), ptr::null());
        gl::CompileShader(fragment);
        gl::GetShaderiv(fragment, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            gl::GetShaderInfoLog(fragment, 512, ptr::null_mut(), info_log.as_mut_ptr());
            return Err(
                format!(
                    "SHADER::FRAGMENT: compilation failed: {}",
                    std::str::from_utf8(bytes_of(&info_log))?
                ).into()
            );
        }
        fragment
    };

    let id = unsafe {
        let id = gl::CreateProgram();
        gl::AttachShader(id, vertex);
        gl::AttachShader(id, fragment);
        gl::LinkProgram(id);
        gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
        if success == 0 {
            gl::GetProgramInfoLog(id, 512, ptr::null_mut(), info_log.as_mut_ptr());
            return Err(
                format!(
                    "SHADER::PROGRAM: linking failed: {}",
                    std::str::from_utf8(bytes_of(&info_log))?
                ).into()
            );
        }
        gl::DeleteShader(vertex);
        gl::DeleteShader(fragment);
        id
    };

    Ok(Shader::new(id))
}