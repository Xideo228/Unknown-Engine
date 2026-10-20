use std::ffi::c_void;

#[repr(C)]
pub struct VkAllocationCallbacks {
    pfnAllocation: *const c_void,
    pfnReallocation: *const c_void,
    pfnFree: *const c_void,
    pfnInternalAllocation: *const c_void,
    pfnInternalFree: *const c_void,
    pUserData: *const c_void
}