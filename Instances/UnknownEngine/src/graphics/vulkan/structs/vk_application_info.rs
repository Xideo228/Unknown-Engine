use std::ffi::{
    c_void,
    c_char
};

#[repr(C)]
pub struct VkApplicationInfo {
    pub sType: u32,
    pub pNext: *const c_void,
    pub pApplicationVersion: u32,
    pub applicationVersion: u32,
    pub pEngineName: *const c_char,
    pub engineVersion: u32,
    pub apiVersion: u32
}