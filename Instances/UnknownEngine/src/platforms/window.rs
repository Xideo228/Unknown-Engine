use std::ffi::c_void;

pub struct Window {
    pub(crate) is_running: bool,
    pub(crate) hwnd: *mut c_void,
    pub(crate) hdc: *mut c_void
}

impl Window {
    pub fn new(hwnd: *mut c_void, hdc: *mut c_void, is_running: bool) -> Self {
        Self { hwnd: hwnd, hdc: hdc, is_running: is_running }
    }
}