use std::ffi::c_void;

pub type VkInstance = *mut c_void;
pub type VkDevice = *mut c_void;
pub type VkPhysicalDevice = *mut c_void;
pub type VkSurfaceKHR = *mut c_void;
pub type VkResult = i32;
pub type VkBool32 = u32;
pub type VkFlags = u32;
pub type VkSampleCountFlagBits = u32;
pub type VkFormat = u32;
pub type VkImageUsageFlagBits = u32;
pub type VkSharingMode = u32;
pub type VkComponentSwizzle = u32;
pub type VkImageAspectFlagBits = u32;
pub type VkAccessFlagBits = u32;
pub type VkPipelineStageFlags = u32;
pub type VkMemoryPropertyFlags = u32;