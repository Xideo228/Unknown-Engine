use std::ffi::c_void;
use crate::platforms::types::HMODULE;

#[link(name = "kernel32")]
unsafe extern "system" {
    pub fn LoadLibraryW(name: *const u16) -> HMODULE;
    pub fn FreeLibrary(module: HMODULE) -> i32;
    pub fn GetProcAddress(module: HMODULE, name: *const u8) -> *mut c_void;
    pub fn GetModuleHandleW(module_name: *const u16) -> HMODULE;
}

pub(crate) fn get_module_handle() -> HMODULE {
    unsafe {
        GetModuleHandleW(std::ptr::null())
    }
}