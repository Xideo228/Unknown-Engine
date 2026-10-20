use std::ffi::c_void;

#[repr(C)]
pub struct VkCommandBufferAllocateInfo {
    sType: u32,
    pNext: *const c_void,
    commandPool: *mut c_void,
    level: u32,
    commandBufferCount: u32
}