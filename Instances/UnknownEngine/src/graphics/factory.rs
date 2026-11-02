use crate::{
    graphics::{
        GraphicsAPI, GraphicsBackend
    }, platforms::Window
};
use super::{
    opengl::{
        wgl,
        gl::*
    }
};

pub struct GraphicsFactory;

impl GraphicsFactory {
    pub fn create(api: GraphicsAPI, win: Window) -> Box<dyn GraphicsBackend> {
        match api {
            GraphicsAPI::OpenGL => {
                wgl::init(&win);
                Box::new(GL::new(win))
            }

            GraphicsAPI::Vulkan => {
                panic!("In current time Vulkan API not support");
                //Box::new(Vulkan::new(win).expect("Failed to initialize Vulkan backend"))
            },
            GraphicsAPI::DirectX11 => todo!()
        }
    }
}