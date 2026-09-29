use crate::{Window, logger::print, platforms::{
    WindowSettings,
    platform::Platform,
    windows::Win32Platform
}};

pub struct WindowManager;

impl WindowManager {
    pub fn create(settings: WindowSettings) -> Window {
        print(0, "Trying to create window");
        print(1, "Support only Windows");

        let mut platform = Win32Platform::new();
        let window = platform.create_window(settings);
        window
    }
}