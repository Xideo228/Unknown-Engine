use std::ffi::c_void;

use crate::platforms::{types::*, windows::structs::*};

#[link(name = "user32")]
unsafe extern "system" {
    pub fn RegisterClassW(class: *const WndClassW) -> u16;
    pub fn CreateWindowExW(ex_style: DWORD, class_name: LPCWSTR, window_name: LPCWSTR,
        style: DWORD, x: i32, y: i32, width: i32, height: i32, parent: HWND, menu: *mut c_void,
        instance: HINSTANCE,param: *mut c_void) -> HWND;
    pub fn ShowWindow(hwnd: HWND, command: i32) -> i32;
    pub fn DefWindowProcW(hwnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT;
    pub fn PeekMessageW(msg: *mut MSG, hwnd: HWND, min: UINT, max: UINT, remove: UINT) -> i32;
    pub fn TranslateMessage(msg: *const MSG) -> i32;
    pub fn DispatchMessageW(msg: *const MSG) -> LRESULT;
    pub fn PostQuitMessage(exit_code: i32);
    pub fn GetDC(hwnd: HWND) -> HDC;
}