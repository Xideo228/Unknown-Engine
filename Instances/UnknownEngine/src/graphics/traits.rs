pub trait GraphicsContext {
    fn make_current(&self);
    fn swap_buffers(&self);
}