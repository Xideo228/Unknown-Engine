use crate::GL;

pub trait GraphicsBackend {
    fn Clear(mask: u32);
    fn ClearColor(r: f32, g: f32, b: f32, A: f32);
    fn GetString(text: u32) -> *const u8;
}

pub trait GraphicsContext {
    fn make_current(&self);
    fn swap_buffers(&self);
}

pub struct OpenGLBackend {
    gl: GL
}