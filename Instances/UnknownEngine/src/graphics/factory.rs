use crate::{GraphicsAPI, Window, graphics::opengl};

pub struct GraphicsFactory {
    api: GraphicsAPI
}

impl GraphicsFactory {
    pub fn create(api: GraphicsAPI, win: Window) {
        match api {
            GraphicsAPI::OpenGL => {
                opengl::init(win);
            }
        }
    }
}