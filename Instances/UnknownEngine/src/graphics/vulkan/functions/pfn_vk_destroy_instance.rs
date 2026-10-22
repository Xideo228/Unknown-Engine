use crate::graphics::vulkan::{
    types::VkInstance,
    structs::VkAllocationCallbacks
};

pub type PfnVkDestroyInstance = unsafe extern "system" fn (
    instance: VkInstance,
    allocator: *const VkAllocationCallbacks
);