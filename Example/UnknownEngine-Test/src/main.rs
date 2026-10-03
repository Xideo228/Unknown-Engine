use std::ffi::{CStr, c_char};

use unknown_engine::*;
use unknown_engine::backend::GraphicsBackend;

fn main() {
    let settings= WindowSettings::default();
    let mut window = WindowManager::create(settings);

    let mut renderer = GraphicsFactory::create(GraphicsAPI::OpenGL, window);

    let raw = renderer.get_string(0x1F02) as *const c_char;

    unsafe {
        let c_str = CStr::from_ptr(raw);

        match c_str.to_str() {
            Ok(rust_str) => println!("Данные из OpenGL: {}", rust_str),
            Err(e)  => println!("Ошибка валидации UTF-8: {}", e)
        }
    }

    while window.is_running() {
        renderer.clear_color(1.0, 0.0, 0.5, 0.0);
        renderer.clear();

        renderer.swap_buffer();
        window.poll_event();
    }
}