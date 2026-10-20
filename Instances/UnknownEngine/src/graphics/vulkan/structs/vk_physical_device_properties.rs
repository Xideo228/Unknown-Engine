use crate::graphics::vulkan::types::VkMemoryPropertyFlags;
use super::{
    vk_queue_family_properties::VkQueueFamilyProperties,
    vk_sparse_properties::VkSparseProperties
};

#[repr(C)]
pub struct VkPhysicalDeviceProperties {
    apiVersion: u32,
    driverVersion: u32,
    vendorID: u32,
    deviceID: u32,
    deviceType: u32,
    pipelineUUID: [u8; 16],
    memoryProperties: [VkMemoryPropertyFlags; 32],
    queueFamilyPropertiesCount: u32,
    pQueueFamilyProperties: *const VkQueueFamilyProperties,
    sparseProperties: VkSparseProperties
}