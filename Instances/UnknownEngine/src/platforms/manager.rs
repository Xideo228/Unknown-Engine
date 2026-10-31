use crate::{Window, logger::*, platforms::{
    WindowSettings,
    platform::Platform,
    windows::Win32Platform
}};

pub struct WindowManager;

impl WindowManager {
    pub fn create(settings: WindowSettings) -> Window {
        print(LevelOfLog::Info, String::from("Trying to create window"));
        print(LevelOfLog::Warning, String::from("Support only Windows"));

        let mut platform = Win32Platform::new();

        let window = platform.create_window(settings);
        window
    }
}