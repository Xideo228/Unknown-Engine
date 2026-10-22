use crate::graphics::vulkan::{
    structs::{
        VkInstanceCreateInfo,
        VkAllocationCallbacks,
    },
    types::{
        VkInstance,
        VkResult
    }
};

pub type PfnVkCreateInstance = unsafe extern "system" fn (
    create_info: *const VkInstanceCreateInfo,
    allocator: *const VkAllocationCallbacks,
    instance: *mut VkInstance
) -> VkResult;