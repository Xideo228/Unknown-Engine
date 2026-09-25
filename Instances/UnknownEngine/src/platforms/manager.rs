use crate::platforms::{
    WindowSettings,
    platform::Platform,
    windows::Win32Platform
};

pub struct WindowManager {
    settings: WindowSettings
}

impl WindowManager {
    pub fn create(settings: WindowSettings) -> Win32Platform {
        let mut platform = Win32Platform::new();
        let window = platform.create_window(settings);
        platform
    }
}