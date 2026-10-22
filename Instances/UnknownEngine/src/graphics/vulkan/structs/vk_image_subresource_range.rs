use crate::graphics::vulkan::types::VkImageAspectFlagBits;

#[repr(C)]
pub struct VkImageSubresourceRange {
    pub aspectMask: VkImageAspectFlagBits,
    pub baseMipLevel: u32,
    pub levelCount: u32,
    pub baseArrayLayer: u32,
    pub layerCount: u32
}