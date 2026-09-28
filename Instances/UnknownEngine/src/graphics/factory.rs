use crate::{GL, GraphicsAPI, Window, graphics::{opengl::{self, wgl_proc_address, wglGetProcAddress}, traits::GraphicsContext}};

pub struct GraphicsFactory;

impl GraphicsFactory {
    pub fn create(api: GraphicsAPI, win: &Window) -> Box<dyn GraphicsContext> {
        match api {
            GraphicsAPI::OpenGL => {
                opengl::init(win);
                unsafe { GL::load(|name| wgl_proc_address(name)); }
                GL
            }
        }
    }
}