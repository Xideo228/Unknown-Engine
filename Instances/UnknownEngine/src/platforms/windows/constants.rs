use crate::platforms::windows::types::{DWORD, UINT};

pub const WS_OVERLAPPEDWINDOW: DWORD = 0x00CF0000;
pub const CW_USERDEFAULT: i32 = 0x80000000u32 as i32;
pub const SW_SHOW: i32 = 5;

pub const WM_DESTROY: UINT = 0x0002;
pub const WM_QUIT: UINT = 0x0012;

pub const PM_REMOVE: u32 = 0x0001;