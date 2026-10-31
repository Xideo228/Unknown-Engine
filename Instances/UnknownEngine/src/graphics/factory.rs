use crate::{
    Window,
    graphics::{
        GraphicsBackend,
        GraphicsAPI
    }
};
use super::{
    opengl::{
        wgl,
        gl::*
    }
};

pub struct GraphicsFactory;

impl GraphicsFactory {
    pub fn create(api: GraphicsAPI, win: Window) -> GL {
        match api {
            GraphicsAPI::OpenGL => {
                wgl::init(&win);
                GL::new(win)
            }

            GraphicsAPI::Vulkan => {
                panic!("In current time Vulkan API not support");
                //Box::new(Vulkan::new(win).expect("Failed to initialize Vulkan backend"))
            },
            GraphicsAPI::DirectX11 => todo!()
        }
    }
}