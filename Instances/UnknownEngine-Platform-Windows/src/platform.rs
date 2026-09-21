use std::ptr::{null, null_mut};
use crate::{
    events::window_proc,
    sys::{
        constants::*,
        user32::*
    },
    window::Win32Window
};
use UnknownEngine_Platform::{
    Platform,
    WindowDescriptor
};

pub struct Win32Platform {
    should_close : bool
}

impl Win32Platform {
    pub fn new() -> Self {
        Self {
            should_close: false
        }
    }
}

impl Platform for Win32Platform {
    type Window = Win32Window;

    fn create_window(&mut self, descriptor: WindowDescriptor) -> Self::Window {
        unsafe {
            let class_name = b"UnknownEngine_Window\0";
            let title = format!("{}\0", descriptor.title);

            let class = WndClassA {
                style: 0,
                lpfn_wnd_proc: Some(window_proc),
                cb_cls_extra: 0,
                cb_wnd_extra: 0,
                h_instance: null_mut(),
                h_icon: null_mut(),
                h_cursor: null_mut(),
                hbr_background: null_mut(),
                lpsz_menu_name: null(),
                lpsz_class_name: class_name.as_ptr()
            };

            RegisterClassA(&class);

            let hwnd = CreateWindowExA(0, class_name.as_ptr(), title.as_ptr(), WS_OVERLAPPEDWINDOW, CW_USEDEFAULT, CW_USEDEFAULT,
                descriptor.width as i32, descriptor.height as i32, null_mut(), null_mut(), null_mut(), null_mut());

            ShowWindow(hwnd, SW_SHOW);
            Win32Window::new(hwnd, descriptor.width, descriptor.height)
        }
    }

    fn poll_events(&mut self) {
        unsafe {
            let mut msg = std::mem::zeroed::<Msg>();

            while PeekMessageA(&mut msg, null_mut(), 0, 0, PM_REMOVE) != 0 {
                if msg.message == WM_QUIT { self.should_close = true; }

                TranslateMessage(&msg);
                DispatchMessageA(&msg);
            }
        }
    }

    fn should_close(&self) -> bool {
        self.should_close
    }
}