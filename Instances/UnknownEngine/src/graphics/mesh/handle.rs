#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MeshHandle(u32);

impl MeshHandle {
    pub(crate) fn new(id: u32) -> Self {
        Self(id)
    }

    pub(crate) fn id(&self) -> u32 {
        self.0
    }
}