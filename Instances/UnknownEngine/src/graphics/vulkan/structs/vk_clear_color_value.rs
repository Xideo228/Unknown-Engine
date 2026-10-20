#[repr(C)]
pub union VkClearColorValue {
    float32: [f32; 4],
    int32: [i32; 4],
    uint32: [u32; 4]
}