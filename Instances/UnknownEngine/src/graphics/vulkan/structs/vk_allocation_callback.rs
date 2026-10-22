use std::ffi::c_void;

#[repr(C)]
pub struct VkAllocationCallbacks {
    pub pfnAllocation: *const c_void,
    pub pfnReallocation: *const c_void,
    pub pfnFree: *const c_void,
    pub pfnInternalAllocation: *const c_void,
    pub pfnInternalFree: *const c_void,
    pub pUserData: *const c_void
}