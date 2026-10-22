use crate::graphics::vulkan::types::*;

#[repr(C)]
pub struct VkAttachmentDescription {
    pub flags: VkFlags,
    pub format: VkFormat,
    pub samples: VkSampleCountFlagBits,
    pub loadOp: u32,
    pub storeOp: u32,
    pub stencilLoadOp: u32,
    pub stencilStoreOp: u32,
    pub initialLayout: u32,
    pub finalLayout: u32
}