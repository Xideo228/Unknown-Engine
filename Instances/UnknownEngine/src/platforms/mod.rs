mod settings;
mod event;
mod manager;
mod platform;
mod window;

pub(crate) mod windows;

pub mod types;

pub use settings::WindowSettings;
pub use manager::WindowManager;
pub use platform::EventPump;
pub use window::Window;