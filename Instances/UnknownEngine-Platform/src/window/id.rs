#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WindowId(u32);

impl WindowId {
    pub(crate) fn new(id: u32) -> Self {
        Self(id)
    }
}