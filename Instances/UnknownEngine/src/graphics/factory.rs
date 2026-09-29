use crate::{GL, GraphicsAPI, Window, graphics::{backend::GraphicsBackend, opengl::self}};

pub struct GraphicsFactory;

impl GraphicsFactory {
    pub fn create(api: GraphicsAPI, win: &Window) -> GL {
        match api {
            GraphicsAPI::OpenGL => {
                opengl::init(win);
                GL::new(win)
            }

            GraphicsAPI::Vulkan => todo!(),
            GraphicsAPI::DirectX11 => todo!()
        }
    }
}