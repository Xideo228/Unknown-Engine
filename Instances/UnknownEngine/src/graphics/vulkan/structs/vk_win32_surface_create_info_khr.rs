use std::ffi::c_void;

#[repr(C)]
pub struct VkWin32SurfaceCreateInfoKHR {
    pub sType: u32,
    pub pNext: *const c_void,
    pub flags: u32,
    pub hinstance: *mut c_void,
    pub hwnd: *mut c_void
}