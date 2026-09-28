use unknown_engine::*;

fn main() {
    let settings= WindowSettings::default();
    let mut window = WindowManager::create(settings);

    GraphicsFactory::create(GraphicsAPI::OpenGL, &window);

    while window.is_running() {
        window.poll_event();
    }
}