#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShaderInstanceHandle {
    index: u32,
    generation: u32
}