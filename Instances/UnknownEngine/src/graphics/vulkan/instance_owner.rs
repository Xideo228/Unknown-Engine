use std::ptr;
use super::{
    types::VkInstance,
    functions::PfnVkDestroyInstance
};

pub struct InstanceOwner {
    pub handle: VkInstance,
    pub destroy: PfnVkDestroyInstance
}

impl Drop for InstanceOwner {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { ((self.destroy)(self.handle, ptr::null())); }
        }
    }
}