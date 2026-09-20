use UnknownEngine_Platform::{
    Platform,
    WindowDescriptor
};
use UnknownEngine_Platform_Windows::Win32Platform;

fn main() {
    let mut platform = Win32Platform::new();

    let _window = platform.create_window(
        WindowDescriptor::new("Unknown Engine", 1280, 720)
    );

    while !platform.should_close() {
        platform.poll_events();
    }
}