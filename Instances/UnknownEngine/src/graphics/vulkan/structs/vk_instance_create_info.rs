use std::ffi::{
    c_void,
    c_char
};
use crate::graphics::vulkan::types::VkFlags;
use super::vk_application_info::VkApplicationInfo;

#[repr(C)]
pub struct VkInstanceCreateInfo {
    pub sType: u32,
    pub pNext: *const c_void,
    pub flags: VkFlags,
    pub pApplicationInfo: *const VkApplicationInfo,
    pub enabledLayerCount: u32,
    pub ppEnabledLayerNames: *const *const c_char,
    pub enabledExtensionCount: u32,
    pub ppEnabledExtensionNames: *const *const c_char
}