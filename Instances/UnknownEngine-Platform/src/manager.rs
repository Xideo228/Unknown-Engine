use std::sync::Mutex;
use std::collections::HashMap;
use std::sync::OnceLock;

use super::{WindowSettings, window::{
    Window,
    WindowId,
}};

struct WindowManagerState {
    windows: HashMap<WindowId, Window>,
    current: Option<WindowId>,
    next_id: u32
}

impl WindowManagerState {
    fn new() -> Self {
        Self {
            windows: HashMap::new(),
            current: None,
            next_id: 0
        }
    }
}

pub struct WindowManager;

static STATE: OnceLock<Mutex<WindowManagerState>> = OnceLock::new();

impl WindowManager {
    fn state() -> &'static Mutex<WindowManagerState> {
        STATE.get_or_init(|| Mutex::new(WindowManagerState::new()))
    }

    pub fn create(settings: WindowSettings) -> Result<WindowId, String> {
        let mut state = Self::state().lock().map_err(|_| "WindowManager state is poisoned".to_string())?;

        let id = WindowId::new(state.next_id);
        state.next_id += 1;

        let window = Window::new(id, settings);
        state.windows.insert(id, window);

        if state.current.is_none() {
            state.current = Some(id);
        }

        println!("Created window '{}' ({}x{})", settings.name, settings.width, settings.height);
        Ok(id)
    }

    pub fn current() -> CurrentWindow {
        let state = Self::state().lock().expect("WindowManager state is poisoned");
        CurrentWindow {
            id: state.current
        }
    }

    pub fn update() {
        todo!()
    }

    pub fn count() -> usize {
        Self::state().lock().map(|state| state.windows.len()).unwrap_or(0)
    }
}