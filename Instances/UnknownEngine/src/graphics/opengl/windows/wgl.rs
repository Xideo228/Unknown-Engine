use std::{ffi::c_void, ptr::null_mut};

use crate::{graphics::opengl::constants::{WGL_CONTEXT_CORE_PROFILE_BIT_ARB, WGL_CONTEXT_MAJOR_VERSION_ARB, WGL_CONTEXT_MINOR_VERSION_ARB, WGL_CONTEXT_PROFILE_MASK_ARB}, platforms::*};
use super::{ PFD::PixelFormatDescriptor, gdi32::* };

#[link(name = "opengl32")]
unsafe extern "system" {
    pub fn wglCreateContext(hdc: HDC) -> HGLRC;
    pub fn wglMakeCurrent(hdc: HDC, context: HGLRC) -> i32;
    pub fn wglDeleteContext(context: HGLRC) -> i32;
    pub fn wglGetCurrentContext() -> HGLRC;
    pub fn wglGetCurrentDC() -> HDC;
    pub fn wglGetProcAddress(name: *const i8) -> *const c_void;
}

pub type WglCreateContextAttribsARB = unsafe extern "system" fn(hdc: HDC, share_context: HGLRC, attribs: *const i32) -> HGLRC;

fn init(win: Window) {
    unsafe {
        let pixel_format = PixelFormatDescriptor {
            size: std::mem::size_of::<PixelFormatDescriptor>() as u16,
            version: 1,
            flags: PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER,
            pixel_type: PFD_TYPE_RGBA,

            color_bits: 32,

            red_bits: 0,
            red_shift: 0,
            green_bits: 0,
            green_shift: 0,
            blue_bits: 0,
            blue_shift: 0,
            alpha_bits: 8,
            alpha_shift: 0,

            accum_bits: 0,
            accum_red_bits: 0,
            accum_green_bits: 0,
            accum_blue_bits: 0,
            accum_alpha_bits: 0,

            depth_bits: 24,
            stencil_bits: 8,

            aux_buffers: 0,
            layer_type: PFD_MAIN_PLANE,

            reserved: 0,
        
            layer_mask: 0,
            visible_mask: 0,
            damage_mask: 0
        };

        let format = ChoosePixelFormat(win.hdc, &pixel_format);

        let temp = wglCreateContext(win.hdc);
        if temp.is_null() { panic!("Failed to create WGL Context"); }

        if wglMakeCurrent(win.hdc, temp) == 0 { panic!(); }

        let proc = load(b"wglCreateContextAttribsARB\0").expect("wglCreateContextAttribsARB is unavailable");
        let create_context_attribs: WglCreateContextAttribsARB = std::mem::transmute(proc);

        let attribs = [
            WGL_CONTEXT_MAJOR_VERSION_ARB, 3,
            WGL_CONTEXT_MINOR_VERSION_ARB, 3,
            WGL_CONTEXT_PROFILE_MASK_ARB, WGL_CONTEXT_CORE_PROFILE_BIT_ARB,
            0
        ];

        let context = create_context_attribs(win.hdc, null_mut(), attribs.as_ptr());
        wglMakeCurrent(win.hdc, context);
        wglDeleteContext(temp);
    }
}

unsafe fn load(name: &'static [u8]) -> Option<*const c_void> {
    let ptr = wglGetProcAddress(name.as_ptr() as *const i8);
    if ptr.is_null() || ptr as usize == 1 || ptr as usize == 2 || ptr as usize == 3 || ptr as isize == -1 {
        None
    } else {
        Some(ptr)
    }
}