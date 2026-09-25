use crate::platforms::WindowSettings;

pub trait Platform {
    type Window;

    fn create_window(&mut self, settings:WindowSettings) -> Self::Window;
    fn poll_event(&mut self);
    fn is_running(&self) -> bool;
}