mod settings;
mod event;
mod manager;
mod platform;

#[cfg(target_os = "windows")]
mod window;

mod android;

pub(crate) mod windows;

pub mod types;

pub use settings::WindowSettings;
pub use manager::WindowManager;
pub use platform::EventPump;
pub use window::Window;