use super::VkExtent2D;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct VkQueueFamilyProperties {
    pub queueFlags: u32,
    pub queueCount: u32,
    pub timestampValidBits: u32,
    pub minImageTransferGranularity: VkExtent2D
}