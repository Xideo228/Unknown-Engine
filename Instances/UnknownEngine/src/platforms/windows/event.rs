use super::user32::{DefWindowProcA, PostQuitMessage};

use super::{types::*, constans::*};

pub(crate) unsafe extern "system" fn window_proc(hwnd: HWnd, msg: UInt, w_param: WParam, l_param: LParam) -> LResult {
    match msg {
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        }

        _ => {
            DefWindowProcA(hwnd, msg, w_param, l_param)
        }
    }
}