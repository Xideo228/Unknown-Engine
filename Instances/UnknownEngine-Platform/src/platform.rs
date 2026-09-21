use crate::windowdescriptor::WindowDescriptor;

pub trait Platform {
    type Window;

    fn create_window(&mut self, descriptor: WindowDescriptor) -> Self::Window;
    fn poll_events(&mut self);
    fn should_close(&self) -> bool;
}