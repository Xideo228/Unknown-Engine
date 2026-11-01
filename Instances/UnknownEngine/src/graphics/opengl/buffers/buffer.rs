use crate::graphics::opengl::{
    constants::*,
    gl::GL
};
use std::{
    ffi::c_void,
    mem::size_of_val
};

pub fn create_buffer(gl: &GL) -> u32 {
    let mut buffer: u32 = 0;
    unsafe { (gl.gen_buffers)(1, &mut buffer); }
    buffer
}

pub fn delete_buffer(gl: &GL, buffer: u32) { unsafe { (gl.delete_buffers)(1, &buffer); } }
pub fn bind_buffer(gl: &GL, buffer: u32) { unsafe { (gl.bind_buffer)(GL_ARRAY_BUFFER, buffer); } }
pub fn bind_index_buffer(gl: &GL, buffer: u32) { unsafe { (gl.bind_buffer)(GL_ELEMENT_ARRAY_BUFFER, buffer); } }
pub fn upload_index_data(gl: &GL, data: &[u32]) { unsafe { (gl.buffer_data)(GL_ELEMENT_ARRAY_BUFFER,
    size_of_val(data) as isize, data.as_ptr() as *const c_void, GL_STATIC_DRAW); } }