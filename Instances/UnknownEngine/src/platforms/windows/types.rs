use std::ffi::c_void;

pub type HMODULE = *mut c_void;
pub type HWND = *mut c_void;
pub type HINSTANCE = HMODULE;
pub type LPCWSTR = *const u16;

pub type UINT = u32;
pub type DWORD = u32;
pub type WPARAM = usize;
pub type LPARAM = isize;
pub type LRESULT = isize;
