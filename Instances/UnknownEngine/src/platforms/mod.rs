mod settings;
mod manager;
mod platform;
mod time;

#[cfg(test)]
mod tests;

#[cfg(target_family = "windows")]
mod window;

#[cfg(target_family = "windows")]
pub(crate) use time::time::get_timestamp;

#[cfg(target_family = "unix")]
pub(crate) use time::time::get_timestamp;

pub(crate) mod windows;
pub(crate) mod types;

pub use settings::WindowSettings;
pub use manager::WindowManager;
pub use platform::EventPump;
pub use window::Window;