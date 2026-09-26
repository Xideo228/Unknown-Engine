use std::ffi::c_void;
use super::helper::load_function;

pub type GlClearColor = unsafe extern "system" fn (f32, f32, f32, f32);
pub type GlClear = unsafe extern "system" fn (u32);
pub type GlGetString = unsafe extern "system" fn (u32) -> *const u8;

pub struct GL {
    pub clear_color: GlClearColor,
    pub clear: GlClear,
    pub get_string: GlGetString
}

impl GL {
    pub unsafe fn load<T>(mut loader: T) -> Self where T: FnMut(&'static [u8]) -> *const c_void {
        let clear_color = load_function(&mut loader, b"glClearColor\0");
        let clear = load_function(&mut loader, b"glClear\0");
        let get_string = load_function(&mut loader, b"glGetString\0");

        Self {
            clear_color, clear, get_string
        }
    }
}