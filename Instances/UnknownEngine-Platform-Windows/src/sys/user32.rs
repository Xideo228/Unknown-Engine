use super::types::*;
use std::ffi::c_void;

#[repr(C)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

#[repr(C)]
pub struct Msg {
    pub hwnd: HWnd,
    pub message: Uint,
    pub w_param: WParam,
    pub l_param: LParam,
    pub time: DWord,
    pub pt: Point
}

#[repr(C)]
pub struct WndClassA {
    pub style: Uint,
    pub lpfn_wnd_proc: Option<unsafe extern "system" fn(HWnd, Uint, WParam, LParam) -> LResult>,
    pub cb_cls_extra: i32,
    pub cb_wnd_extra: i32,
    pub h_instance: HInstance,
    pub h_icon: HIcon,
    pub h_cursor: HCursor,
    pub hbr_background: HBrush,
    pub lpsz_menu_name: *const u8,
    pub lpsz_class_name: *const u8
}

#[link(name = "user32")]
unsafe extern "system" {
    pub fn RegisterClassA(class: *const WndClassA) -> u16;
    pub fn CreateWindowExA(ex_style: DWord, class_name: *const u8, window_name: *const u8, style: DWord, x: i32, y: i32,
        width: i32, height: i32, parent: HWnd, menu: *mut c_void, instance: HInstance, param: *mut c_void) -> HWnd;
    pub fn ShowWindow(hwnd: HWnd, command: i32) -> i32;
    pub fn DefWindowProcA(hwnd: HWnd, msg: Uint, w_param: WParam, l_param: LParam) -> LResult;
    pub fn PeekMessageA(msg: *mut Msg, hwnd: HWnd, min: Uint, max: Uint, remove: Uint) -> i32;
    pub fn TranslateMessage(msg: *const Msg) -> i32;
    pub fn DispatchMessageA(msg: *const Msg) -> LResult;
    pub fn PostQuitMessage(exit_code: i32);
}