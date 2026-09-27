use std::{ffi::c_void, mem::transmute};
use super::windows::wglGetProcAddress;

pub type GlClearColor = unsafe extern "system" fn (f32, f32, f32, f32);
pub type GlClear = unsafe extern "system" fn (u32);
pub type GlGetString = unsafe extern "system" fn (u32) -> *const u8;

pub struct GL {
    pub clear_color: GlClearColor,
    pub clear: GlClear,
    pub get_string: GlGetString
}

impl GL {
    pub unsafe fn load() -> Self {
        let clear_color = transmute(wglGetProcAddress(b"glClearColor\0".as_ptr() as *const i8));
        let clear = transmute(wglGetProcAddress(b"glClear\0".as_ptr() as *const i8));
        let get_string = transmute(wglGetProcAddress(b"glGetString\0".as_ptr() as *const i8));

        Self {
            clear_color, clear, get_string
        }
    }
}

unsafe fn load_function<T, F>(loader: &mut F, name: &'static [u8]) -> T where F: FnMut(&'static [u8]) -> *const c_void {
    let ptr = loader(name);
    if ptr.is_null() {
        panic!("Failed to load OpenGL function: {:?}", name);
    }
    std::mem::transmute_copy(&ptr)
}