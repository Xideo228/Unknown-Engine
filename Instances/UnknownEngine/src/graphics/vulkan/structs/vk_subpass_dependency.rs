use crate::graphics::vulkan::types::{
    VkPipelineStageFlags,
    VkAccessFlagBits,
    VkFlags
};

#[repr(C)]
pub struct VkSubpassDependency {
    srcSubpass: u32,
    dstSubpass: u32,
    srcStageMask: VkPipelineStageFlags,
    dstStageMask: VkPipelineStageFlags,
    srcAccessMask: VkAccessFlagBits,
    dstAccessMask: VkAccessFlagBits,
    dependencyFlags: VkFlags
}