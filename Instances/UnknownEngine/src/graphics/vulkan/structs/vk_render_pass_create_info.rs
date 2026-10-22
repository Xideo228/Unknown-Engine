use crate::graphics::vulkan::types::VkFlags;
use super::{
    vk_subpass_description::VkSubpassDescription,
    vk_attachment_description::VkAttachmentDescription,
    vk_subpass_dependency::VkSubpassDependency
};
use std::ffi::c_void;

#[repr(C)]
pub struct VkRenderPassCreateInfo {
    pub sType: u32,
    pub pNext: *const c_void,
    pub flags: VkFlags,
    pub attachmentCount: u32,
    pub pAttachment: *const VkAttachmentDescription,
    pub subpassCount: u32,
    pub pSubpasses: *const VkSubpassDescription,
    pub dependencyCount: u32,
    pub pDependencies: *const VkSubpassDependency
}