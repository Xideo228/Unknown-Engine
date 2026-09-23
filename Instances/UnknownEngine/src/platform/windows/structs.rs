use std::os::raw::c_void;
use crate::platform::windows::types::*;

#[repr(C)]
pub struct WndClassA {
    cb_size: UInt,
    style: UInt,
    wnd_proc: Option<unsafe extern "system" fn(HWnd, UInt, WParam, LParam) -> LResult>,
    cb_cls_extra: i32,
    cb_wnd_extra: i32,
    h_instance: HInstance,
    h_icon: *mut c_void,
    h_cursor: *mut c_void,
    hbr_background: *mut c_void,
    menu_name: LPcwstr,
    class_name: LPcwstr,
    h_icon_sm: *mut c_void
}

#[repr(C)]
pub struct MSG {
    hwnd: HWnd,
    message: UInt,
    w_param: WParam,
    l_param: LParam,
    time: DWord,
    pt_x: i32,
    pt_y: i32
}

#[repr(C)]
pub struct Point {
    x: i32,
    y: i32
}