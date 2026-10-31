mod settings;
mod event;
mod manager;
mod platform;

#[cfg(target_os = "windows")]
mod window;

pub(crate) mod windows;
pub(crate) mod types;

pub use settings::WindowSettings;
pub use manager::WindowManager;
pub use platform::EventPump;
pub use window::Window;