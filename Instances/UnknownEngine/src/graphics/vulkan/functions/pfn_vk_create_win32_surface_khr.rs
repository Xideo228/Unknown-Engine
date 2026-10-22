use crate::graphics::vulkan::{
    types::{
        VkSurfaceKHR,
        VkInstance,
        VkResult
    },
    structs::{
        VkWin32SurfaceCreateInfoKHR,
        VkAllocationCallbacks
    }
};

pub type PfnVkCreateWin32SurfaceKHR = unsafe extern "system" fn (
    instance: VkInstance,
    create_info: *const VkWin32SurfaceCreateInfoKHR,
    allocator: *const VkAllocationCallbacks,
    surface: *mut VkSurfaceKHR
) -> VkResult;