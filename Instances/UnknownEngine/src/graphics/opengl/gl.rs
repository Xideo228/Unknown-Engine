use std::{ffi::c_void,
    mem::{
        transmute,
        transmute_copy
    }
};
use crate::graphics::{backend::GraphicsBackend, opengl::constants::GL_COLOR_BUFFER_BIT};

pub type GlClearColor = unsafe extern "system" fn (f32, f32, f32, f32);
pub type GlClear = unsafe extern "system" fn (u32);
pub type GlGetString = unsafe extern "system" fn (u32) -> *const u8;

pub struct GL {
    pub clear_color: GlClearColor,
    pub clear: GlClear,
    pub get_string: GlGetString
}

impl GL {
    pub unsafe fn load<F>(mut loader: F) -> Self where F: FnMut(&'static [u8]) -> *const c_void {
        Self {
            clear_color: load_function(&mut loader, b"glClearColor\0"),
            clear: load_function(&mut loader, b"glClear\0"),
            get_string: load_function(&mut loader, b"glGetString\0")
        }
    }

    pub unsafe fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        (self.clear_color)(r, g, b, a);
    }

    pub unsafe fn clear(&self) {
        (self.clear)(GL_COLOR_BUFFER_BIT);
    }

    pub unsafe fn get_string(&self, name: u32) -> *const u8 {
        (self.get_string)(name)
    }
}

unsafe fn load_function<T, F>(loader: &mut F, name: &'static [u8]) -> T where F: FnMut(&'static [u8]) -> *const c_void {
    let ptr = loader(name);
    if ptr.is_null() {
        panic!(
            "Failed to load OpenGL function: {}",
            String::from_utf8_lossy(&name[..name.len() - 1])
        );
    }
    transmute_copy(&ptr)
}