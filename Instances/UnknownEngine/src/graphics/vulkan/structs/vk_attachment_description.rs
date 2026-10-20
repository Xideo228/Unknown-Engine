use crate::graphics::vulkan::types::*;

#[repr(C)]
pub struct VkAttachmentDescription {
    flags: VkFlags,
    format: VkFormat,
    samples: VkSampleCountFlagBits,
    loadOp: u32,
    storeOp: u32,
    stencilLoadOp: u32,
    stencilStoreOp: u32,
    initialLayout: u32,
    finalLayout: u32
}