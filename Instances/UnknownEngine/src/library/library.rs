use std::ffi::c_void;

pub trait Library {
    fn load(name: &str) -> Option<Self> where Self: Sized;
    fn get_proc_address(&self, name: &str) -> Option<*mut c_void>;
}