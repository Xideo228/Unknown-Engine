<<<<<<< HEAD
#[derive(Debug, Clone)]
=======
#[derive(Clone)]
>>>>>>> 59178fed7e2bc0ce9546f9d2c5fd55c51b9741bf
pub struct WindowSettings {
    pub name: String,
    pub title: String,

    pub width: u32,
    pub height: u32,

    pub resizable: bool,
    pub decorated: bool,

    pub vsync: bool,
    pub fullscreen: bool
}

impl Default for WindowSettings {
    fn default() -> Self 
    {
        Self { 
            name: String::from("unknown-engine"),
            title: String::from("Unknown Engine"),
            width: 1280,
            height: 720,
            resizable: true,
            decorated: true,
            vsync: true,
            fullscreen: false
        }
    }
}