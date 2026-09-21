use crate::window::WindowId;

pub struct Window {
    pub(crate) id: WindowId
}

impl Window {
    pub(crate) fn new(id: WindowId) -> Self { Self { id } }
    pub fn id(&self) -> WindowId { self.id }
}