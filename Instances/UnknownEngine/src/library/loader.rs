use std::fmt::Error;
use crate::HMODULE;

pub struct LibraryLoader {
    handle: HMODULE
}

impl LibraryLoader {
    pub fn load(name: &str) -> Result<Self, Error> {
        todo!();
    }
}