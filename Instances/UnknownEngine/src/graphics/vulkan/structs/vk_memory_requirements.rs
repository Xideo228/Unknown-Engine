#[repr(C)]
pub struct VkMemoryRequirement {
    size: u64,
    alignment: u64,
    memoryTypeBits: u32
}