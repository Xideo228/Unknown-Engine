use std::ffi::c_void;

#[repr(C)]
pub struct VkCommandBufferAllocateInfo {
    pub sType: u32,
    pub pNext: *const c_void,
    pub commandPool: *mut c_void,
    pub level: u32,
    pub commandBufferCount: u32
}