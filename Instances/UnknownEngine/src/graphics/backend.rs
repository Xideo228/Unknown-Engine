pub trait GraphicsBackend {
    fn make_current(&self);
    fn swap_buffers(&self);

    fn Clear(buffer: u32);
    fn ClearColor(r: f32, g: f32, b: f32, A: f32);
    fn GetString(text: u32) -> *const u8;
}