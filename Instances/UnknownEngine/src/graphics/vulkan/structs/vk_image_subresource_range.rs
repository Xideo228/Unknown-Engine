use crate::graphics::vulkan::types::VkImageAspectFlagBits;

#[repr(C)]
pub struct VkImageSubresourceRange {
    aspectMask: VkImageAspectFlagBits,
    baseMipLevel: u32,
    levelCount: u32,
    baseArrayLayer: u32,
    layerCount: u32
}