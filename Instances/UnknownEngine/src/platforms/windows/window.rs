use std::ptr::{null, null_mut};
use crate::platforms::{windows::{
    constans::*, event::window_proc, structs::*, types::HWnd, user32::*
}, *};
use crate::platforms::platform::Platform;

pub struct Win32Platform {
    pub is_running: bool
}

pub struct Win32Window {
    pub(crate) settings: WindowSettings
}

impl Win32Platform {
    pub fn new() -> Self {
        Self { is_running: true }
    }
}

impl Win32Window {
    pub fn new(settings: WindowSettings) -> Self {
        Self { settings: settings }
    }
}

impl Platform for Win32Platform {
    type Window = Win32Window;

    fn create_window(&mut self, settings: WindowSettings) -> Self::Window {
        unsafe {
            let class_name = format!("UnknownEngine_{}\0", settings.name);
            let title = format!("{}\0", settings.title);
            let class = WndClassA {
                style: 0,
                wnd_proc: Some(window_proc),
                cb_cls_extra: 0,
                cb_wnd_extra: 0,
                h_instance: null_mut(),
                h_icon: null_mut(),
                h_cursor: null_mut(),
                hbr_background: null_mut(),
                menu_name: null(),
                class_name: class_name.as_ptr()
            };

            RegisterClassA(&class);

            let hwnd = CreateWindowExA(0, class_name.as_ptr(), title.as_ptr(),
                WS_OVERLAPPEDWINDOW, CW_USERDEFAULT, CW_USERDEFAULT, settings.width as i32, settings.height as i32,
                null_mut(), null_mut(), null_mut(), null_mut());

            ShowWindow(hwnd, SW_SHOW);
            Win32Window::new(settings)
        }
    }

    fn poll_event(&mut self) {
        unsafe {
            let mut msg = std::mem::zeroed::<MSG>();

            while PeekMessageA(&mut msg, null_mut(), 0, 0, PM_REMOVE) != 0 {
                if msg.message == WM_QUIT { self.is_running = false; }

                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    }

    fn is_running(&self) -> bool {
        self.is_running
    }
}