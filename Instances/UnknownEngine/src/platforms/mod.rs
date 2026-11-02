mod settings;
mod event;
mod manager;
mod platform;
mod time;

#[cfg(target_os = "windows")]
mod window;

#[cfg(target_os = "windows")]
pub(crate) use time::time::get_timestamp;

#[cfg(target_os = "unix")]
pub(crate) use time::time::get_timestamp;

pub(crate) mod windows;
pub(crate) mod types;

pub use settings::WindowSettings;
pub use manager::WindowManager;
pub use platform::EventPump;
pub use window::Window;