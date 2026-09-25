use std::os::raw::c_void;
use crate::platforms::windows::types::*;

#[repr(C)]
pub(crate) struct WndClassA {
    pub style: UInt,
    pub wnd_proc: Option<unsafe extern "system" fn(HWnd, UInt, WParam, LParam) -> LResult>,
    pub cb_cls_extra: i32,
    pub cb_wnd_extra: i32,
    pub h_instance: HInstance,
    pub h_icon: *mut c_void,
    pub h_cursor: *mut c_void,
    pub hbr_background: *mut c_void,
    pub menu_name: LPcwstr,
    pub class_name: LPcwstr
}

#[repr(C)]
pub(crate) struct MSG {
    pub hwnd: HWnd,
    pub message: UInt,
    pub w_param: WParam,
    pub l_param: LParam,
    pub time: DWord,
    pub pt_x: i32,
    pub pt_y: i32
}

#[repr(C)]
pub struct Point {
    x: i32,
    y: i32
}