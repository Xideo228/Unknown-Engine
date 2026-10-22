#[repr(C)]
pub struct VkMemoryRequirement {
    pub size: u64,
    pub alignment: u64,
    pub memoryTypeBits: u32
}