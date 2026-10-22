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
    },
    vulkan::{
        vk::*
    }
};

pub struct GraphicsFactory;

impl GraphicsFactory {
    pub fn create(api: GraphicsAPI, win: Window) -> Box <dyn GraphicsBackend> {
        match api {
            GraphicsAPI::OpenGL => {
                wgl::init(&win);
                Box::new(GL::new(win))
            }

            GraphicsAPI::Vulkan => {
                Box::new(Vulkan::new(win).expect("Failed to initialize Vulkan backend"))
            },
            GraphicsAPI::DirectX11 => todo!()
        }
    }
}