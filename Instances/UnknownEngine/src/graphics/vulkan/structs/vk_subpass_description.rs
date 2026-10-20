use crate::graphics::vulkan::types::VkFlags;

#[repr(C)]
pub struct VkSubpassDescription {
    flags: VkFlags,
    pipelineBindPoint: u32,
    pColorAttachments: *const u32,
    colorAttachmentCount: u32,
    pDepthStencilAttachments: *const u32,
    pPreserveAttachments: *const u32,
    preserveAttachmentCount: u32
}