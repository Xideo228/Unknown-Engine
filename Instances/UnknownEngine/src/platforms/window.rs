use std::ffi::c_void;

pub struct Window {
    pub hwnd: *mut c_void,
    pub hdc: *mut c_void
}

impl Window {
    pub fn new(hwnd: *mut c_void, hdc: *mut c_void) -> Self {
        Self { hwnd, hdc }
    }
}