use crate::graphics::vulkan::types::VkMemoryPropertyFlags;
use super::{
    vk_queue_family_properties::VkQueueFamilyProperties,
    vk_sparse_properties::VkSparseProperties
};

#[repr(C)]
pub struct VkPhysicalDeviceProperties {
    pub apiVersion: u32,
    pub driverVersion: u32,
    pub vendorID: u32,
    pub deviceID: u32,
    pub deviceType: u32,
    pub pipelineUUID: [u8; 16],
    pub memoryProperties: [VkMemoryPropertyFlags; 32],
    pub queueFamilyPropertiesCount: u32,
    pub pQueueFamilyProperties: *const VkQueueFamilyProperties,
    pub sparseProperties: VkSparseProperties
}