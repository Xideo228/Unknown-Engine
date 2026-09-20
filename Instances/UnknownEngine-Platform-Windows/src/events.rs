use crate::sys::{
    types::*,
    constants::*,
    user32::*
};

pub unsafe extern "system" fn window_proc(hwnd: HWnd, message: Uint, w_param: WParam, l_param: LParam) -> LResult {
    match message {
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        }

        _ => {
            DefWindowProcA(hwnd, message, w_param, l_param)
        }
    }
}