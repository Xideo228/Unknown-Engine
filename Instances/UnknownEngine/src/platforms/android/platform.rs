use std::ffi::c_void;

pub struct AndroidPlatform {
    native_window: *mut c_void,
    width: u32,
    height: u32
}

