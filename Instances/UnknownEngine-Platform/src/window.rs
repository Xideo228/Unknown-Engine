use crate::settings::WindowSettings;

pub trait Platform {
    type Window;

    fn create_window(settings: WindowSettings) -> Result<Self::Window, String>;
    fn update();
}