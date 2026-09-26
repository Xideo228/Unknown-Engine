use crate::{GraphicsAPI, Window};

pub struct GraphycsFactory {
    api: GraphicsAPI
}

impl GraphycsFactory {
    fn init(api: GraphicsAPI, win: Window) {
        match api {
            GraphicsAPI::OpenGL => {
                
            }
        }
    }
}