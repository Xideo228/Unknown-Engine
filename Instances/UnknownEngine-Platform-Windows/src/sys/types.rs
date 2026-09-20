use std::ffi::c_void;

pub type HWnd = *mut c_void;
pub type HInstance = *mut c_void;
pub type HCursor = *mut c_void;
pub type HIcon = *mut c_void;
pub type HBrush = *mut c_void;

pub type WParam = usize;
pub type LParam = isize;
pub type LResult = isize;

pub type Uint = u32;
pub type DWord = u32;