use std::{str, sync::Mutex};
use std::collections::HashMap;
use std::sync::OnceLock;

use super::{
    Window,
    WindowId,
    WindowSettings
};

struct WindowManagerState {
    window: HashMap<WindowId, Window>,
    next_id: u32
}

impl WindowManagerState {
    fn new() -> Self {
        Self {
            window: HashMap::new(),
            next_id: 0
        }
    }
}

pub struct WindowManager;

static STATE: OnceLock<Mutex<WindowManagerState>> = OnceLock::new();

impl WindowManager {
    fn state() -> &'static Mutex<WindowManagerState> {
        STATE.get_or_init(|| {
            Mutex::new(WindowManagerState::new())
        })
    }
}