#[cfg(target_family = "windows")]
pub mod time {
    #[repr(C)]
    struct SystemTime {
        year: u16,
        month: u16,
        day_of_week: u16,
        day: u16,
        hour: u16,
        minute: u16,
        second: u16,
        millisecond: u16
    }

    unsafe extern "system" { fn GetLocalTime(time: *mut SystemTime); }

    pub fn get_timestamp() -> String {
        let mut time = SystemTime { year: 0, month: 0, day_of_week: 0,
            day: 0, hour: 0, minute: 0, second: 0, millisecond: 0 };
        unsafe { GetLocalTime(&mut time); }

        format!("{:02}.{:02}.{:04} {:02}:{:02}:{:02}", time.day, time.month, time.year, time.hour, time.minute, time.second)
    }
}

#[cfg(target_family = "unix")]
pub mod time {
    use std::ffi::{
        c_char, c_void,
        CStr
    };

    type TimeT = i64;

    #[repr(C, align(8))]
    struct TmStorage([u8; 128]);

    unsafe extern "system" {
        fn localtime_r(time: *const TimeT, result: *mut c_void) -> *mut c_void;
        fn strftime(buffer: *mut c_char, max_size: usize, format: *const c_char, time: *const c_void) -> usize;
    }

    pub fn get_timestamp() -> String {
        let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH).unwrap_or_default();

        let timestamp = now.as_secs() as TimeT;

        let mut tm = TmStorage([0; 128]);
        let mut buffer = [0u8; 32];

        unsafe {
            let result = localtime_r(&timestamp, tm.0.as_mut_ptr() as *mut c_void);
            if result.is_null() { return "00.00.0000 00:00:00".to_string(); }

            let format = b"%d.%m.%Y %H:%M:%S\0";

            let len = strftime(buffer.as_mut_ptr() as *mut c_char, buffer.len(),
                format.as_ptr() as *const c_char, result);
            
            if len == 0 { return "00.00.0000 00:00:00".to_string(); }

            CStr::from_ptr(buffer.as_ptr() as *const c_char).to_string_lossy().into_owned()
        }
    }
}