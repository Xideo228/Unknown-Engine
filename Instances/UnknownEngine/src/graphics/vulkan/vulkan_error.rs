use crate::graphics::vulkan::types::VkResult;
use std::{
    fmt::Display,
    error::Error
};

#[derive(Debug)]
pub enum VulkanError {
    LibraryLoad,
    MissingFunction(&'static str),
    ApiCall(&'static str, VkResult),
    NoPhysicalDevices,
    NoSuitableQueue,
    UnsupportedPlatform
}

impl Display for VulkanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LibraryLoad => write!(f, "Failed to load Vulkan library"),
            Self::MissingFunction(name) => write!(f, "Missing Vulkan function: {name}"),
            Self::ApiCall(name, result) => write!(f, "{name} failed with result: {result}"),
            Self::NoPhysicalDevices => write!(f, "No Vulkan phusical devices found"),
            Self::NoSuitableQueue => write!(f, "No queue supports both Graphics and Present"),
            Self::UnsupportedPlatform => write!(f, "This Vulkan backend currently unsupport your platfrom")
        }
    }
}

impl Error for VulkanError {}