use crate::{Window, platforms::{
    WindowSettings,
    platform::Platform,
    windows::Win32Platform
}};

pub struct WindowManager {
    settings: WindowSettings
}

impl WindowManager {
    pub fn create(settings: WindowSettings) -> Window {
        let mut platform = Win32Platform::new();
        let window = platform.create_window(settings);
        window
    }
}