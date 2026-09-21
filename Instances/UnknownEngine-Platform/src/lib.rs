mod windowdescriptor;
mod platform;
mod manager;
mod settings;

pub mod window;

pub use settings::WindowSettings;
pub use manager::WindowManager;
pub use platform::Platform;
pub use windowdescriptor::WindowDescriptor;