use crate::graphics::vulkan::types::VkInstance;
use std::ffi::{
    c_char,
    c_void
};

pub type PfnVkGetInstanceProcAddr = unsafe extern "system" fn (
    instance: VkInstance,
    name: *const c_char
) -> *const c_void;