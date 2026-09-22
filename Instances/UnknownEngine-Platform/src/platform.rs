use crate::{
    handle::WindowHandle,
    settings::{
        self,
        WindowSettings
    },
    window
};

#[unsafe(no_mangle)]
pub extern "C" fn platform_create_window(settings: *const WindowSettings, window: *mut WindowHandle) -> i32 {
    if settings.is_null() || window.is_null() { return 0; }
    println!("Window platform: create window");
    1
}

#[unsafe(no_mangle)]
pub extern "C" fn platform_update_window(window: *mut WindowHandle) {
    println!("Window platform: update window");
}

#[unsafe(no_mangle)]
pub extern "C" fn platform_destroy_window(window: *mut WindowHandle) {
    println!("Window platform: destroy window");
}