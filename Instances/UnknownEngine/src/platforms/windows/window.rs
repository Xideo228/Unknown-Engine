use std::ptr::{null, null_mut};
use crate::platforms::{platform::*, window::Window, windows::{
    constants::*, event::window_proc, kernel32::get_module_handle, structs::*, user32::*
}, *};

pub struct Win32Platform {
    is_running: bool
}

impl Win32Platform {
    pub fn new() -> Self {
        Self { is_running: true }
    }
}

impl Platform for Win32Platform {
    type Window = Window;

    fn create_window(&mut self, settings: WindowSettings) -> Self::Window {
        unsafe {
            let class_name: Vec<u16> = format!("UnknownEngine_{}\0", settings.name).encode_utf16().collect();
            let title: Vec<u16> = format!("{}\0", settings.title).encode_utf16().collect();
            let class = WndClassW {
                style: 0,
                lpfn_wnd_proc: Some(window_proc),
                cb_cls_extra: 0,
                cb_wnd_extra: 0,
                h_instance: get_module_handle(),
                h_icon: null_mut(),
                h_cursor: null_mut(),
                hbr_background: null_mut(),
                lpsz_menu_name: null(),
                lpsz_class_name: class_name.as_ptr()
            };

            RegisterClassW(&class);

            let hwnd = CreateWindowExW(0, class_name.as_ptr(), title.as_ptr(),
                WS_OVERLAPPEDWINDOW, CW_USERDEFAULT, CW_USERDEFAULT, settings.width as i32, settings.height as i32,
                null_mut(), null_mut(), null_mut(), null_mut());
            let hdc = GetDC(hwnd);

            ShowWindow(hwnd, SW_SHOW);
            Window::new(hwnd, hdc)
        }
    }
}

impl EventPump for Win32Platform {
    fn poll_event(&mut self) {
        unsafe {
            let mut msg = std::mem::zeroed::<MSG>();

            while PeekMessageW(&mut msg, null_mut(), 0, 0, PM_REMOVE) != 0 {
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