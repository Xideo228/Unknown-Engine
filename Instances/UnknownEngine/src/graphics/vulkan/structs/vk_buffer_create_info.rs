use std::ffi::c_void;
use crate::graphics::vulkan::types::{
    VkFlags,
    VkSharingMode
};

#[repr(C)]
pub struct VkBufferCreateInfo {
    sType: u32,
    pNext: *const c_void,
    flags: VkFlags,
    size: u64,
    usage: VkFlags,
    sharingMode: VkSharingMode,
    queueFamilyIndexCount: u32,
    pQueueFamilyIndices: *const u32
}