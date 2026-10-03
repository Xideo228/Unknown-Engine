use std::{ffi::c_void, mem::transmute_copy};
use super::windows::wgl::*;
use crate::{
    Window, graphics::{
        backend::GraphicsBackend, opengl::{constants::*, gdi32}
    }, library::windows::WinLibrary
};

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
        Self {
            window: win,

            get_string: load_function(&mut |name| { wgl_get_proc_address(name) }, b"glGetString\0"),
            clear: load_function(&mut |name| { wgl_get_proc_address(name) }, b"glClear\0"),
            clear_color: load_function(&mut |name| { wgl_get_proc_address(name) }, b"glClearColor\0"),
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

    fn swap_buffer(&mut self) {
        unsafe { gdi32::SwapBuffers(self.window.hdc); }
    }
}

fn load_function<T, F>(loader: &mut F, name: &'static [u8]) -> T where F: FnMut(&'static [u8]) -> *const c_void {
    let mut ptr = loader(name);

    if ptr.is_null() {
        let library = WinLibrary::load("opengl32").expect("Failed to load opengl32.dll");
        ptr = WinLibrary::get_proc_address(&library, name).expect("Failed to load");
    }

    if ptr.is_null() {
        panic!(
            "Failed to load OpenGL function: {}",
            String::from_utf8_lossy(&name[..name.len() - 1])
        );
    }
    unsafe { transmute_copy(&ptr) }
}