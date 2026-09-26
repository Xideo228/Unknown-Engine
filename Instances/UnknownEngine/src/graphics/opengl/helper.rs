use std::{ffi::c_void, mem::transmute_copy};

pub unsafe fn load_function<T, F>(loader: &mut F, name: &'static [u8]) -> T where F: FnMut(&'static [u8]) -> *const c_void {
    let ptr = loader(name);
    if ptr.is_null() {
        panic!("Failed to load OpenGL function: {:?}", name);
    }
    std::mem::transmute_copy(&ptr)
}