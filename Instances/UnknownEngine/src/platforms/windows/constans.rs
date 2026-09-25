use crate::platforms::windows::types::{DWord, UInt};

pub const WS_OVERLAPPEDWINDOW: DWord = 0x00CF0000;
pub const CW_USERDEFAULT: i32 = 0x80000000u32 as i32;
pub const SW_SHOW: i32 = 5;

pub const WM_DESTROY: UInt = 0x0002;
pub const WM_QUIT: UInt = 0x0012;

pub const PM_REMOVE: u32 = 0x0001;