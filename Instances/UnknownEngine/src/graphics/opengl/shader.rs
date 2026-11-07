use crate::{
    graphics::shader::*,
    logger::*
};
use super::{
    gl::GL,
    constants::*
};
use std::{
    ffi::{
        CStr, CString, c_char
    }, ptr::{
        null,
        null_mut
    }
};

pub(crate) struct GLShaderProgram { id: u32 }

impl GLShaderProgram {
    pub(crate) const fn new(id: u32) -> Self { Self { id } }

    pub(crate) const fn id(&self) -> u32 { self.id }
}

pub(crate) fn create_shader_program(gl: &GL, vertex_source: &str, fragment_source: &str) -> Result<u32, String> {
    let vertex = compile_shader(&gl, GL_VERTEX_SHADER, vertex_source.to_string())?;
    let fragment = match compile_shader(&gl, GL_FRAGMENT_SHADER, fragment_source.to_string()) {
        Ok(shader) => shader,
        Err(error) => {
            unsafe { (gl.functions.delete_shader)(vertex); }
            return Err(error);
        }
    };

    let program = unsafe { (gl.functions.create_program)() };
    if program == 0 {
        unsafe {
            (gl.functions.delete_shader)(fragment);
            (gl.functions.delete_shader)(vertex);
        };

        return Err("Failed to create shader program".into());
    }

    unsafe {
        (gl.functions.attach_shader)(program, vertex);
        (gl.functions.attach_shader)(program, fragment);
        (gl.functions.link_program)(program);
    };

    let mut status = 0;

    unsafe {
        (gl.functions.get_program_iv)(program, GL_LINK_STATUS, &mut status);

        (gl.functions.delete_shader)(vertex);
        (gl.functions.delete_shader)(fragment);
    };

    if status == 0 {
        let error = get_program_error(&gl, program);
        unsafe { (gl.functions.delete_program)(program); }

        return Err(error);
    }

    Ok(program)
}

pub(crate) fn use_shader(gl: &GL, program: u32) { unsafe { (gl.functions.use_program)(program); } }

fn compile_shader(gl: &GL, shader_type: u32, source: String) -> Result<u32, String> {
    let source = CString::new(source).map_err(|_| "Shader source contains null byte".to_string())?;

    let shader = unsafe { (gl.functions.create_shader)(shader_type) };
    if shader == 0 { return Err("Failed to create Shader".into()); }

    let source_ptr = source.as_ptr();

    unsafe {
        (gl.functions.shader_source)(shader, 1, &source_ptr, null());
        (gl.functions.compile_shader)(shader);
    }

    let mut status = 0;

    unsafe { (gl.functions.get_shader_iv)(shader, GL_COMPILE_STATUS, &mut status); }

    if status == 0 {
        let error = get_shader_error(gl, shader);
        unsafe { (gl.functions.delete_shader)(shader); }
        return Err(error);
    }
    Ok(shader)
}

fn get_shader_error(gl: &GL, shader: u32) -> String {
    let mut lenght = 0;

    unsafe { (gl.functions.get_shader_iv)(shader, GL_INFO_LOG_LENGTH, &mut lenght); };
    if lenght <= 1 { return "No shader info log available".to_string() }

    let mut buffer = vec![0u8; lenght as usize];
    unsafe {
        (gl.functions.get_shader_info_log)(shader, lenght, null_mut(), buffer.as_mut_ptr() as *mut c_char);
        CStr::from_ptr(buffer.as_ptr() as *const c_char).to_string_lossy().into_owned()
    }
}

fn get_program_error(gl: &GL, program: u32) -> String {
    let mut lenght = 0;

    unsafe { (gl.functions.get_program_iv)(program, GL_INFO_LOG_LENGTH, &mut lenght); };
    if lenght <= 1 { return "Unknown program compile error".into(); }

    let mut buffer = vec![0u8; lenght as usize];
    unsafe {
        (gl.functions.get_program_info_log)(program, lenght, null_mut(), buffer.as_mut_ptr() as *mut c_char);
        CStr::from_ptr(buffer.as_ptr() as *const c_char).to_string_lossy().into_owned()
    }
}

fn get_uniform_location(gl: &GL, handle: ShaderHandle, name: &str) -> Result<i32, String> {
    let shader = gl.shaders.get(&handle.id()).ok_or_else(|| {
        format!("Invalid shader handle: {}", handle.id())
    })?;

    let name = CString::new(name).map_err(|_| "Uniform name contains null byte".to_string())?;
    let location = unsafe { (gl.functions.get_uniform_location)(shader.id(), name.as_ptr()) };

    Ok(location)
}

fn set_uniform_mat4(gl: &mut GL, handle: ShaderHandle, name: &str, value: [f32; 16]) {
    let location = match get_uniform_location(gl, handle, name) {
        Ok(location) => location,
        Err(error) => {
            log(LevelOfLog::Error, error);
            return;
        }
    };

    if location == -1 { return; }

    unsafe { (gl.functions.uniform_matrix_4fv)(location, 1, 0, value.as_ptr()); }
}

fn set_uniform_float(gl: &mut GL, handle: ShaderHandle, name: &str, value: f32) {
    let location = match get_uniform_location(gl, handle, name) {
        Ok(location) => location,
        Err(error) => {
            log(LevelOfLog::Error, error);
            return;
        }
    };

    if location == -1 { return; }
    unsafe { (gl.functions.uniform_1f)(location, value); }
}