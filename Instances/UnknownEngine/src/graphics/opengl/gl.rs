use std::{
    ffi::{
        CStr,
        c_char,
        c_void
    }, mem::transmute_copy
};
use super::windows::wgl::*;
use crate::{
    Window,
    graphics::{
        backend::GraphicsBackend,
        opengl::{
            constants::*,
            gdi32
        }
    },
    library::Library
};

#[cfg(target_os = "windows")]
const OPENGL: &str = "opengl32.dll";

type GlClearColor = unsafe extern "system" fn (f32, f32, f32, f32);
type GlClear = unsafe extern "system" fn (u32);
type GlGetString = unsafe extern "system" fn (u32) -> *const u8;

pub struct GL {
    window: Window,

    clear_color: GlClearColor,
    clear: GlClear,
    get_string: GlGetString
}

impl GL {
    pub fn new(win: Window) -> Self {
        let opengl32 = Library::new(OPENGL).expect("Failed to load library");

        Self {
            window: win,

            get_string: load_function(&opengl32, &mut |name| { wgl_get_proc_address(name) }, b"glGetString\0"),
            clear: load_function(&opengl32, &mut |name| { wgl_get_proc_address(name) }, b"glClear\0"),
            clear_color: load_function(&opengl32, &mut |name| { wgl_get_proc_address(name) }, b"glClearColor\0"),
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

    fn get_string(&mut self, text: u32) -> &str {
        unsafe {
            let ptr = (self.get_string)(text) as *const c_char;
            let c_str = CStr::from_ptr(ptr);
            match c_str.to_str() {
                Ok(result) => result,
                Err(_) => ""
            }
        }
    }

    fn swap_buffer(&mut self) {
        unsafe { gdi32::SwapBuffers(self.window.hdc); }
    }
}

fn load_function<T, F>(lib: &Library, loader: &mut F, name: &'static [u8]) -> T where F: FnMut(&'static [u8]) -> *const c_void {
    let mut ptr = loader(name);

    if ptr.is_null() {
        ptr = lib.get_function(name).expect("Failed to load");
    }

    if ptr.is_null() {
        panic!(
            "Failed to load OpenGL function: {}",
            String::from_utf8_lossy(&name[..name.len() - 1])
        );
    }
    unsafe { transmute_copy(&ptr) }
}