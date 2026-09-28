use crate::{GL, GraphicsAPI, Window, graphics::{backend::OpenGLBackend, opengl::{self, wgl_proc_address, wglGetProcAddress}}};

pub struct GraphicsFactory;

impl GraphicsFactory {
    pub fn create(api: GraphicsAPI, win: &Window) -> GL {
        match api {
            GraphicsAPI::OpenGL => {
                opengl::init(win);
                let gl = unsafe{ GL::load(|name| wgl_proc_address(name)) };
                gl
            }
        }
    }
}