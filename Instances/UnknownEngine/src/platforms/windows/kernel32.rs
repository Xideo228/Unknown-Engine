use super::types::HMODULE;

#[link(name = "kernel32")]
unsafe extern "system" {
    pub fn GetModuleHandleW(module_name: *const u16) -> HMODULE;
}

pub(crate) fn get_module_handle() -> HMODULE {
    unsafe {
        GetModuleHandleW(std::ptr::null())
    }
}