use crate::graphics::vulkan::types::{
    VkPhysicalDevice,
    VkSurfaceKHR,
    VkBool32,
    VkResult
};

pub type PfnVkGetPhysicalDeviceSurfaceSupportKHR = unsafe extern "system" fn (
    physical_device: VkPhysicalDevice,
    queue_family_index: u32,
    surface: VkSurfaceKHR,
    supported: *mut VkBool32
) -> VkResult;