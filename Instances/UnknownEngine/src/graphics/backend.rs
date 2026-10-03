pub trait GraphicsBackend {
    fn clear(&mut self);
    fn clear_color(&mut self, r: f32, g: f32, b: f32, a: f32);
    fn get_string(&mut self, text: u32) -> &str;
    fn swap_buffer(&mut self);
}