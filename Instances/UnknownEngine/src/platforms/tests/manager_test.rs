use crate::platforms::{
    EventPump, WindowManager, WindowSettings
};

#[test]
fn test_creation() {
    let set = WindowSettings::default();
    let win = WindowManager::create(set);

    assert!(!win.hwnd.is_null());
    assert!(win.is_running());
}