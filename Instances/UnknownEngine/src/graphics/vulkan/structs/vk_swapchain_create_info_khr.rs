use std::ffi::c_void;
use super::vk_extent_2d::VkExtent2D;
use crate::graphics::vulkan::types::*;

#[repr(C)]
pub struct VkSwapchainCreateInfoKHR {
    sType: u32,
    pNext: *const c_void,
    surface: VkSurfaceKHR,
    minImageCount: u32,
    imageFormat: VkFormat,
    colorSpace: u32,
    imageExtent: VkExtent2D,
    imageArrayLayers: u32,
    imageUsage: VkImageUsageFlagBits,
    sharingMode: VkSharingMode,
    queueFamilyIndexCount: u32,
    pQueueFamilyIndices: *const u32,
    preTransform: u32,
    compositeAlpha: u32,
    presentMode: u32,
    clipped: VkBool32,
    oldSwapchain: VkSurfaceKHR
}