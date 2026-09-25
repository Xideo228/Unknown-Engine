use super::user32::{DefWindowProcW, PostQuitMessage};

use super::{types::*, constants::*};

pub(crate) unsafe extern "system" fn window_proc(hwnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    match msg {
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        }

        _ => {
            DefWindowProcW(hwnd, msg, w_param, l_param)
        }
    }
}