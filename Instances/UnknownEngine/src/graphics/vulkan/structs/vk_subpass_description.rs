use crate::graphics::vulkan::types::VkFlags;

#[repr(C)]
pub struct VkSubpassDescription {
    pub flags: VkFlags,
    pub pipelineBindPoint: u32,
    pub pColorAttachments: *const u32,
    pub colorAttachmentCount: u32,
    pub pDepthStencilAttachments: *const u32,
    pub pPreserveAttachments: *const u32,
    pub preserveAttachmentCount: u32
}