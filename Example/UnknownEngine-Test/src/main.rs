use unknown_engine::*;

fn main() {
    let settings= WindowSettings::default();
    let mut window = WindowManager::create(settings);

    let mut renderer = GraphicsFactory::create(GraphicsAPI::OpenGL, &window);

    while window.is_running() {
        window.poll_event();
    }
}