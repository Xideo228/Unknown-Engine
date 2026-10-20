use std::ffi::c_void;
use crate::graphics::vulkan::types::VkFlags;
use super::vk_application_info::VkApplicationInfo;

#[repr(C)]
pub struct VkInstanceCreateInfo {
    sType: u32,
    pNext: *const c_void,
    flags: VkFlags,
    pApplicationInfo: *const VkApplicationInfo,
    enabledLayerCount: u32,
    ppEnabledLayerNames: *const i8,
    enabledExtensionCount: u32,
    ppEnabledExtensionNames: *const i8
}