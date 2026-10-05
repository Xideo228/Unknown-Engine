use std::ffi::c_void;
use super::windows::WinLibrary;

#[cfg(target_os = "windows")]
type PlatformLibrary = WinLibrary;

pub struct Library {
    handle: PlatformLibrary
}

impl Library {
    pub fn new(name: &str) -> Option<Self> {
        Some(Self {
            handle: PlatformLibrary::new(name)?
        })
    }

    pub fn get_function(&self, name: &[u8]) -> Option<*const c_void> {
        self.handle.get_function(name)
    }
}