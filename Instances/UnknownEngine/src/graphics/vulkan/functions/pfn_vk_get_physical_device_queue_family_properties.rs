use crate::graphics::vulkan::{
    types::VkPhysicalDevice,
    structs::VkQueueFamilyProperties
};

pub type PfnVkGetPhysicalDeviceQueueFamilyProperties = unsafe extern "system" fn (
    physical_device: VkPhysicalDevice,
    count: *mut u32,
    properties: *mut VkQueueFamilyProperties
);