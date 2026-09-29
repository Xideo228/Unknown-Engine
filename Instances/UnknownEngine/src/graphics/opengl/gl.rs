use std::{ffi::c_void, mem::transmute_copy};
use crate::{Window, graphics::{backend::GraphicsBackend, opengl::constants::*}};

pub type GlClearColor = unsafe extern "system" fn (f32, f32, f32, f32);
pub type GlClear = unsafe extern "system" fn (u32);
pub type GlGetString = unsafe extern "system" fn (u32) -> *const u8;

pub struct GL {
    pub clear_color: GlClearColor,
    pub clear: GlClear,
    pub get_string: GlGetString
}

impl GL {
    pub fn new(_window: &Window) -> Self {
        unsafe {
            Self {
                clear: load_function(&mut |name| {super::wgl_get_proc_address(name)}, b"glClear\0"),
                clear_color: load_function(&mut |name| {super::wgl_get_proc_address(name)}, b"glClearColor\0"),
                get_string: load_function(&mut |name| {super::wgl_get_proc_address(name)}, b"glGetString\0"),
            }
        }
    }
}

impl GraphicsBackend for GL {
    fn clear_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        unsafe { (self.clear_color)(r, g, b, a); }
    }

    fn clear(&mut self) {
        unsafe { (self.clear)(GL_COLOR_BUFFER_BIT); }
    }

    fn get_string(&mut self, text: u32) -> *const u8 {
        unsafe { (self.get_string)(text) }
    }
}   

fn load_function<T, F>(loader: &mut F, name: &'static [u8]) -> T where F: FnMut(&'static [u8]) -> *const c_void {
    let ptr = loader(name);
    if ptr.is_null() {
        panic!(
            "Failed to load OpenGL function: {}",
            String::from_utf8_lossy(&name[..name.len() - 1])
        );
    }
    unsafe { transmute_copy(&ptr) }
}