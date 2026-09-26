use crate::platforms::*;
use super::PFD::PixelFormatDescriptor;

#[link(name = "gdi32")]
unsafe extern "system" {
    pub fn ChoosePixelFormat(hdc: HDC, descriptor: *const PixelFormatDescriptor) -> i32;
    pub fn SetPixelFormat(hdc: HDC, format: i32, descriptor: *const PixelFormatDescriptor) -> i32;
    pub fn SwapBuffers(hdc: HDC) -> i32;
}