use std::ffi::c_void;

pub struct Window {
    hwnd: *mut c_void,
    hdc: *mut c_void
}