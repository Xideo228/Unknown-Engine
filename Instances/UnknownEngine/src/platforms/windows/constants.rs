use crate::platforms::windows::types::{DWORD, UINT};

pub const WS_OVERLAPPEDWINDOW: DWORD = 0x00CF0000;
pub const CW_USERDEFAULT: i32 = 0x80000000u32 as i32;
pub const SW_SHOW: i32 = 5;

pub const WM_DESTROY: UINT = 0x0002;
pub const WM_QUIT: UINT = 0x0012;

pub const PM_REMOVE: u32 = 0x0001;

pub const PFD_DRAW_TO_WINDOW: DWORD = 0x00000004;
pub const PFD_SUPPORT_OPENGL: DWORD = 0x00000020;
pub const PFD_DOUBLEBUFFER: DWORD = 0x00000001;
pub const PFD_TYPE_RGBA: u8 = 0;
pub const PFD_MAIN_PLANE: u8 = 0;