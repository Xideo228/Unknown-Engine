use std::{ffi::{CString, OsStr, c_void}, os::windows::ffi::OsStrExt};
use crate::{
    //library::library::Library,
    platforms::windows::kernel32::*,
    types::HMODULE
};

pub struct WinLibrary {
    handle: HMODULE
}

impl WinLibrary {
    pub fn load(name: &str) -> Option<Self> {
        let wide: Vec<u16> = OsStr::new(name).encode_wide().chain(Some(0)).collect();
        let handle = unsafe { LoadLibraryW(wide.as_ptr()) };

        if handle.is_null() {
            None
        } else {
            Some(Self{ handle })
        }
    }

    pub fn get_proc_address(&self, name: &str) -> Option<*mut c_void> {
        let name = CString::new(name).ok()?;
        let address = unsafe { GetProcAddress(self.handle, name.as_ptr()) };

        if address.is_null() {
            None
        } else {
            Some(address)
        }
    }
}

impl Drop for WinLibrary {
    fn drop(&mut self) {
        unsafe { FreeLibrary(self.handle); }
    }
}