use crate::graphics::vulkan::types::VkFlags;
use super::{
    vk_subpass_description::VkSubpassDescription,
    vk_attachment_description::VkAttachmentDescription,
    vk_subpass_dependency::VkSubpassDependency
};
use std::ffi::c_void;

#[repr(C)]
pub struct VkRenderPassCreateInfo {
    sType: u32,
    pNext: *const c_void,
    flags: VkFlags,
    attachmentCount: u32,
    pAttachment: *const VkAttachmentDescription,
    subpassCount: u32,
    pSubpasses: *const VkSubpassDescription,
    dependencyCount: u32,
    pDependencies: *const VkSubpassDependency
}