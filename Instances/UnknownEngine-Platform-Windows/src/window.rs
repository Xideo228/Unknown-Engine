use crate::sys::types::HWnd;

pub struct Win32Window {
    pub(crate) handle: HWnd,
    pub(crate) width: u32,
    pub(crate) height: u32
}

impl Win32Window {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub(crate) fn new(handle: HWnd, width: u32, height: u32) -> Self {
        Self {
            handle, width, height
        }
    }
}