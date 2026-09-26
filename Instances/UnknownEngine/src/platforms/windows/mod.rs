mod window;
mod constants;
mod structs;
mod user32;
mod kernel32;
mod event;

pub use window::Win32Platform;

pub(crate) use { kernel32::*, constants::*};