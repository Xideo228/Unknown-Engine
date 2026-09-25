use std::ffi::c_void;

use crate::platforms::windows::{structs::*, types::*};

#[link(name = "user32")]
unsafe extern "system" {
    pub fn RegisterClassA(class: *const WndClassA) -> u16;
    pub fn CreateWindowExA(ex_style: DWord, class_name: *const u8, window_name: *const u8,
        style: DWord, x: i32, y: i32, width: i32, height: i32, parent: HWnd, menu: *mut c_void,
        instance: HInstance,param: *mut c_void) -> HWnd;
    pub fn ShowWindow(hwnd: HWnd, command: i32) -> i32;
    pub fn DefWindowProcA(hwnd: HWnd, msg: UInt, w_param: WParam, l_param: LParam) -> LResult;
    pub fn PeekMessageA(msg: *mut MSG, hwnd: HWnd, min: UInt, max: UInt, remove: UInt) -> i32;
    pub fn TranslateMessage(msg: *const MSG) -> i32;
    pub fn DispatchMessageW(msg: *const MSG) -> LResult;
    pub fn PostQuitMessage(exit_code: i32);
}