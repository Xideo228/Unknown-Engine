use std::ptr;
use super::{
    functions::PfnVkDestroySurfaceKHR,
    types::{
        VkInstance,
        VkSurfaceKHR
    }
};

pub struct SurfaceOwner {
    pub instance: VkInstance,
    pub handle: VkSurfaceKHR,
    pub destroy: PfnVkDestroySurfaceKHR
}

impl Drop for SurfaceOwner {
    fn drop(&mut self) {
        if self.handle != 0 {
            unsafe {
                (self.destroy)(self.instance, self.handle, ptr::null());
            }
        }
    }
}