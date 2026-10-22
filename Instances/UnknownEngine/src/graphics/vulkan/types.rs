use std::ffi::c_void;

pub type VkQueue = *mut c_void;
pub type VkDevice = *mut c_void;
pub type VkInstance = *mut c_void;
pub type VkCommandBuffer = *mut c_void;
pub type VkPhysicalDevice = *mut c_void;

pub type VkImage = u64;
pub type VkFence = u64;
pub type VkImageView = u64;
pub type VkSemaphore = u64;
pub type VkSurfaceKHR = u64;
pub type VkRenderPass = u64;
pub type VkFrameBuffer = u64;
pub type VkCommandPool = u64;
pub type VkSwapchainKHR = u64;

pub type VkFlags = u32;
pub type VkResult = i32;
pub type VkBool32 = u32;
pub type VkFormat = u32;
pub type VkDeviceSize = u64;
pub type VkSharingMode = u32;
pub type VkAccessFlags = u32;
pub type VkAccessFlagBits = u32;
pub type VkImageUsageFlags = u32;
pub type VkComponentSwizzle = u32;
pub type VkImageUsageFlagBits = u32;
pub type VkPipelineStageFlags = u32;
pub type VkImageAspectFlagBits = u32;
pub type VkSampleCountFlagBits = u32;
pub type VkMemoryPropertyFlags = u32;