use std::ffi::c_void;
use super::vk_extent_2d::VkExtent2D;
use crate::graphics::vulkan::types::*;

#[repr(C)]
pub struct VkSwapchainCreateInfoKHR {
    pub sType: u32,
    pub pNext: *const c_void,
    pub surface: VkSurfaceKHR,
    pub minImageCount: u32,
    pub imageFormat: VkFormat,
    pub colorSpace: u32,
    pub imageExtent: VkExtent2D,
    pub imageArrayLayers: u32,
    pub imageUsage: VkImageUsageFlagBits,
    pub sharingMode: VkSharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
    pub preTransform: u32,
    pub compositeAlpha: u32,
    pub presentMode: u32,
    pub clipped: VkBool32,
    pub oldSwapchain: VkSurfaceKHR
}