use std::ffi::c_void;
use super::{
    library::Library,
    //windows::WinLibrary
};

pub struct _LibraryLoader {
    handle: Box<dyn Library>
}

impl _LibraryLoader {
    pub fn _load(_lib: &str, _name: &str) -> Option<*mut c_void> {
        todo!()
    }
}