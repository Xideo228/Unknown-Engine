use crate::graphics::vulkan::types::{
    VkPipelineStageFlags,
    VkAccessFlagBits,
    VkFlags
};

#[repr(C)]
pub struct VkSubpassDependency {
    pub srcSubpass: u32,
    pub dstSubpass: u32,
    pub srcStageMask: VkPipelineStageFlags,
    pub dstStageMask: VkPipelineStageFlags,
    pub srcAccessMask: VkAccessFlagBits,
    pub dstAccessMask: VkAccessFlagBits,
    pub dependencyFlags: VkFlags
}