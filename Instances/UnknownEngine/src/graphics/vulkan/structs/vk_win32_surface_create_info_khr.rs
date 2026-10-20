use std::ffi::c_void;

#[repr(C)]
pub struct VkWin32SurfaceCreateInfoKHR {
    sType: u32,
    pNext: *const c_void,
    flags: u32,
    hInstance: *mut c_void,
    hWnd: *mut c_void
}