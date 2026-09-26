use std::os::raw::c_void;
use crate::platforms::types::*;

#[repr(C)]
pub(crate) struct WndClassW {
    pub style: UINT,
    pub lpfn_wnd_proc: Option<unsafe extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT>,
    pub cb_cls_extra: i32,
    pub cb_wnd_extra: i32,
    pub h_instance: HINSTANCE,
    pub h_icon: *mut c_void,
    pub h_cursor: *mut c_void,
    pub hbr_background: *mut c_void,
    pub lpsz_menu_name: LPCWSTR,
    pub lpsz_class_name: LPCWSTR
}

#[repr(C)]
pub(crate) struct MSG {
    pub hwnd: HWND,
    pub message: UINT,
    pub w_param: WPARAM,
    pub l_param: LPARAM,
    pub time: DWORD,
    pub pt_x: i32,
    pub pt_y: i32
}

#[repr(C)]
pub struct Point {
    pub x: i32,
    pub y: i32
}