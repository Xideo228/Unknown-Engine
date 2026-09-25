use std::ffi::c_void;

pub type HModule = *mut c_void;
pub type HWnd = *mut c_void;
pub type HInstance = HWnd;
pub type LPcwstr = *const u8;

pub type UInt = u32;
pub type DWord = u32;
pub type WParam = usize;
pub type LParam = isize;
pub type LResult = isize;