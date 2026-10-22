use std::ffi::c_void;
use crate::graphics::vulkan::types::{
    VkFlags,
    VkSharingMode
};

#[repr(C)]
pub struct VkBufferCreateInfo {
    pub sType: u32,
    pub pNext: *const c_void,
    pub flags: VkFlags,
    pub size: u64,
    pub usage: VkFlags,
    pub sharingMode: VkSharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32
}