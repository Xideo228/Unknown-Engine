use unknown_engine::*;
use unknown_engine::graphics::*;

fn main() {
    let settings= WindowSettings::default();
    let mut window = WindowManager::create(settings);

    let mut renderer = GraphicsFactory::create(GraphicsAPI::OpenGL, window);

    println!("{}", renderer.get_string(0x1F02));

    while window.is_running() {
        renderer.clear_color(0.5, 0.0, 1.0, 0.0);
        renderer.clear();

        renderer.swap_buffer();
        window.poll_event();
    }
}