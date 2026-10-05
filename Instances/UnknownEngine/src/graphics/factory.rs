use crate::{
    Window,
    graphics::{
        GraphicsBackend,
        GraphicsAPI
    }
};
use super::opengl::{
    wgl,
    gl::*,
};

pub struct GraphicsFactory;

impl GraphicsFactory {
    pub fn create(api: GraphicsAPI, win: Window) -> GL {
        match api {
            GraphicsAPI::OpenGL => {
                wgl::init(&win);
                GL::new(win)
            }

            GraphicsAPI::Vulkan => todo!(),
            GraphicsAPI::DirectX11 => todo!()
        }
    }
}