use crate::graphics::vulkan::{
    structs::VkAllocationCallbacks,
    types::{
        VkInstance,
        VkSurfaceKHR
    }
};

pub type PfnVkDestroySurfaceKHR = unsafe extern "system" fn (
    instance: VkInstance,
    surface: VkSurfaceKHR,
    allocator: *const VkAllocationCallbacks
);