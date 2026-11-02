use crate::graphics::opengl::{
    constants::*,
    gl::GL
};
use std::{
    ffi::c_void,
    mem::size_of_val
};

pub fn create_vertex_array(gl: &GL) -> u32 {
    let mut buffer: u32 = 0;
    unsafe { (gl.functions.gen_vertex_arrays)(1, &mut buffer) }
    buffer
}

pub fn delete_vertex_array(gl: &GL, buffer: u32) { unsafe { (gl.functions.delete_vertex_arrays)(1, &buffer); } }
pub fn bind_vertex_array(gl: &GL, buffer: u32) { unsafe { (gl.functions.bind_vertex_array)(buffer) } }
pub fn upload_vertex_data(gl: &GL, data: &[f32]) { unsafe { (gl.functions.buffer_data)(GL_ARRAY_BUFFER,
    size_of_val(data) as isize, data.as_ptr() as *const c_void, GL_STATIC_DRAW); } }

pub fn vertex_attribute(gl: &GL, index: u32, size: i32, stride: i32, offset: usize) {
    unsafe {
        (gl.functions.enable_vertex_attrib_array)(index);
        (gl.functions.vertex_attrib_pointer)(index, size, GL_FLOAT, 0, stride, offset as *const c_void);
    }
}