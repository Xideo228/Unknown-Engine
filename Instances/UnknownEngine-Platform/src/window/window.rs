pub struct Window {
    pub(crate) id: u32
}

impl Window {
    pub(crate) fn new(id: u32) -> Self { Self { id } }
    pub fn id(&self) -> u32 { self.id }
}