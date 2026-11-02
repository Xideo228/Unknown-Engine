#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShaderHandle(u32);

impl ShaderHandle {
    pub(crate) fn new(id: u32) -> Self { Self(id) }
    pub(crate) fn id(&self) -> u32 { self.0 }
}