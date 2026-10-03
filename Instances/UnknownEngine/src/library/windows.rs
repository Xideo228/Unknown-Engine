use std::{
    ffi::{
        OsStr,
        c_void
    },
    os::windows::ffi::OsStrExt
};
use crate::{
    library::library::Library,
    platforms::windows::kernel32::*,
    types::HMODULE
};

pub struct WinLibrary {
    handle: HMODULE
}

impl Library for WinLibrary {
    fn load(name: &str) -> Option<Self> {
        let wide: Vec<u16> = OsStr::new(name).encode_wide().chain(Some(0)).collect();
        let handle = unsafe { LoadLibraryW(wide.as_ptr()) };

        if handle.is_null() {
            None
        } else {
            Some(Self{ handle })
        }
    }

    fn get_proc_address(&self, name: &[u8]) -> Option<*const c_void> {
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