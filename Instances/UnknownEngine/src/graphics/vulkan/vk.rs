use std::{
    ffi::{
        c_char,
        c_void,
        CString
    },
    ptr
};
use crate::{
    Window, graphics::GraphicsBackend, library::Library
};
use super::{
    vulkan_error::VulkanError,
    surface_owner::SurfaceOwner,
    instance_owner::InstanceOwner,
    types::*,
    functions::*,
    structs::*,
    constants::*
};

pub struct Vulkan {
    _surface: SurfaceOwner,
    _instance: InstanceOwner,

    physical_device: VkPhysicalDevice,
    queue_family_index: u32,

    clear_color: [f32; 4],

    _library: Library
}

impl Vulkan {
    pub fn new(window: Window) -> Result<Self, VulkanError> {
        #[cfg(windows)]
        { Self::new_windows(window) }

        #[cfg(not(windows))]
        {
            let _ = window;
            Err(VulkanError::UnsupportedPlatform)
        }
    }

    #[cfg(windows)]
    fn new_windows(window: Window) -> Result<Self, VulkanError> {
        unsafe {
            use crate::graphics::vulkan::constants::{VK_STRUCTURE_TYPE_APPLICATION_INFO, VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO, VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR};

            let library = Library::new("vulkan-1.dll").ok_or(VulkanError::LibraryLoad)?;
            let get_instance_proc_addr: PfnVkGetInstanceProcAddr = load_export(&library, b"vkGetInstanceProcAddr\0", "vkGetInstanceProcAddr")?;

            let app_name = CString::new("Unknown Engine").unwrap();
            let engine_name = CString::new("Unknown Engine").unwrap();

            let app_info = VkApplicationInfo {
                sType: VK_STRUCTURE_TYPE_APPLICATION_INFO,
                pNext: ptr::null(),
                pApplicationName: app_name.as_ptr(),
                applicationVersion: 1,
                pEngineName: engine_name.as_ptr(),
                engineVersion: 1,

                apiVersion: 1 << 22
            };

            let extensions = [
                CString::new("VK_KHR_surface").unwrap(),
                CString::new("VK_KHR_win32_surface").unwrap()
            ];

            let extension_names: Vec<*const c_char> = extensions.iter()
                .map(|extension| extension.as_ptr())
                .collect();

            let create_info = VkInstanceCreateInfo {
                sType: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
                pNext: ptr::null(),
                flags: 0,
                pApplicationInfo: &app_info,
                enabledLayerCount: 0,
                ppEnabledLayerNames: ptr::null(),
                enabledExtensionCount: extension_names.len() as u32,
                ppEnabledExtensionNames: extension_names.as_ptr()
            };

            let create_instance: PfnVkCreateInstance = load_global_fn(get_instance_proc_addr, b"vkCreateInstance\0", "vkCreateInstance")?;

            let mut instance: VkInstance = ptr::null_mut();

            check(
                create_instance(&create_info, ptr::null(), &mut instance),
                "vkCreateInstance"
            )?;

            let destroy_instance: PfnVkDestroyInstance = load_instance_fn(get_instance_proc_addr, instance, b"vkDestroyInstance\0", "vkdestroyInstance")?;
            let instance_owner = InstanceOwner {
                handle: instance,
                destroy: destroy_instance
            };

            let enumerate_physical_devices: PfnVkEnumeratePhysicalDevices = load_instance_fn(get_instance_proc_addr, instance, b"vkEnumeratePhysicalDevices\0", "vkEnumeratePhysicalDevice")?;
            let get_queue_family_properties: PfnVkGetPhysicalDeviceQueueFamilyProperties = load_instance_fn(get_instance_proc_addr, instance, b"vkGetPhysicalDeviceQueueFamilyProperties\0", "vkGetPhysicalDeviceQueueFamilyProperties")?;
            let create_surface: PfnVkCreateWin32SurfaceKHR = load_instance_fn(get_instance_proc_addr, instance, b"vkCreateWin32SurfaceKHR\0", "vkCreateWin32SurfaceKHR")?;
            let destroy_surface: PfnVkDestroySurfaceKHR = load_instance_fn(get_instance_proc_addr, instance, b"vkDestroySurfaceKHR\0", "vkDestroySurfaceKHR")?;
            let get_surface_support: PfnVkGetPhysicalDeviceSurfaceSupportKHR = load_instance_fn(get_instance_proc_addr, instance, b"vkGetPhysicalDeviceSurfaceSupportKHR\0", "vkGetPhysicalDeviceSurfaceSupportKHR")?;
            
            let surface_info = VkWin32SurfaceCreateInfoKHR {
                sType: VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR,
                pNext: ptr::null(),
                flags: 0,
                hinstance: crate::platforms::windows::kernel32::get_module_handle(),
                hwnd: window.hwnd
            };

            let mut surface: VkSurfaceKHR = 0;

            check(
                create_surface(instance, &surface_info, ptr::null(), &mut surface),
                "vkCreateWin32SurfaceKHR"
            )?;

            let surface_owner = SurfaceOwner { instance, handle: surface, destroy: destroy_surface };
            let mut device_count = 0;

            check(
                enumerate_physical_devices(instance, &mut device_count, ptr::null_mut()),
                "vkEnumeratePhysicalDevices(count)"
            )?;

            if device_count == 0 {
                return Err(VulkanError::NoPhysicalDevices);
            }

            let mut devices: Vec<VkPhysicalDevice> = vec![ptr::null_mut(); device_count as usize];

            check(
                enumerate_physical_devices(instance, &mut device_count, devices.as_mut_ptr()),
                "vkEnumeratePhysicalDevices(list)"
            )?;

            let mut selected = None;

            'devices: for physical_device in devices {
                let mut queue_count = 0;

                get_queue_family_properties(physical_device, &mut device_count, ptr::null_mut());

                if queue_count == 0 { continue; }

                let mut queues = vec![VkQueueFamilyProperties::default(); queue_count as usize];

                get_queue_family_properties(physical_device, &mut queue_count, queues.as_mut_ptr());

                for (index, queue) in queues.iter().enumerate() {
                    if queue.queueCount == 0 { continue; }
                    if queue.queueFlags & VK_QUEUE_GRAPHICS_BIT == 0 { continue; }

                    let mut present_supported: VkBool32 = 0;

                    let result = get_surface_support(physical_device, index as u32, surface, &mut present_supported);
                    if result != VK_SUCCESS { continue; }

                    if present_supported != 0 {
                        selected = Some((physical_device, index as u32));
                        break 'devices;
                    }
                }
            }

            let Some((physical_device, queue_family_index)) = selected
            else {
                return Err(VulkanError::NoSuitableQueue);
            };

            return Ok(Self {
                _surface: surface_owner,
                _instance: instance_owner,
                physical_device,
                queue_family_index,
                clear_color: [0.05, 0.05, 0.08, 1.0],
                _library: library,
            });
        }
    }

    pub fn set_clear_color(&mut self, r: f32, g: f32, b: f32, a: f32) { self.clear_color = [r, g, b, a] }
    pub fn selected_queue_family(&self) -> u32 { self.queue_family_index }
    pub fn has_physical_device(&self) -> bool { !self.physical_device.is_null() }
}

impl GraphicsBackend for Vulkan {
    fn clear(&mut self) {
        todo!()
    }

    fn clear_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.set_clear_color(r, g, b, a);
    }

    fn get_string(&mut self, text: u32) -> &str {
        "Vulkan"
    }

    fn swap_buffer(&mut self) {
        todo!()
    }
}

fn load_export<T: Copy>(library: &Library, name: &'static [u8], display_name: &'static str) -> Result<T, VulkanError> {
    let address = library.get_function(name)
        .filter(|address| !address.is_null())
        .ok_or(VulkanError::MissingFunction(display_name))?;
    unsafe { Ok(std::mem::transmute_copy(&address)) }
}

fn load_global_fn<T: Copy>(get_instance_proc_addr: PfnVkGetInstanceProcAddr, name: &'static [u8], display_name: &'static str) -> Result<T, VulkanError> {
    let address = unsafe { get_instance_proc_addr(ptr::null_mut(), name.as_ptr().cast()) };
    
    if address.is_null() {
        return Err(VulkanError::MissingFunction(display_name));
    }

    unsafe { Ok(std::mem::transmute_copy(&address)) }
}

fn load_instance_fn<T: Copy>(get_instance_proc_addr: PfnVkGetInstanceProcAddr, instance: VkInstance, name: &'static [u8], display_name: &'static str) -> Result<T, VulkanError> {
    let address = unsafe { get_instance_proc_addr(instance, name.as_ptr().cast()) };

    if address.is_null() {
        return Err(VulkanError::MissingFunction(display_name));
    }

    unsafe { Ok(std::mem::transmute_copy(&address)) }
}

fn check(result: VkResult, operation: &'static str) -> Result<(), VulkanError> {
    if result == VK_SUCCESS {
        Ok(())
    } else {
        Err(VulkanError::ApiCall(operation, result))
    }
}