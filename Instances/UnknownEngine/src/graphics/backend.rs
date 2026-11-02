use crate::graphics::{
    mesh::*, shader::*
};

pub trait GraphicsBackend {
    fn begin_frame(&mut self);
    fn end_frame(&mut self);
    
    fn set_clear_color(&mut self, r: f32, g: f32, b: f32, a: f32);

    fn get_string(&mut self, text: u32) -> &str;

    fn create_shader(&mut self, vertex_source: &str, fragment_source: &str) -> Result<ShaderHandle, String>;
    fn destroy_shader(&mut self, handle: ShaderHandle);
    fn use_shader(&mut self, handle: ShaderHandle);

    fn create_mesh(&mut self, data: &MeshData) -> Result<MeshHandle, String>;
    fn draw_mesh(&mut self, handle: MeshHandle);
    fn destroy_mesh(&mut self, handle: MeshHandle);
}