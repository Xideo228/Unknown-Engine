use crate::graphics::vulkan::types::{
    VkInstance,
    VkPhysicalDevice,
    VkResult
};

pub type PfnVkEnumeratePhysicalDevices = unsafe extern "system" fn (
    instance: VkInstance,
    count: *mut u32,
    devices: *mut VkPhysicalDevice
) -> VkResult;