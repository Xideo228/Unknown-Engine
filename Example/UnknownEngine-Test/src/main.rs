use unknown_engine::*;

fn main() {
    let settings= WindowSettings::default();
    let mut window = WindowManager::create(settings);

    let mut Render = GraphicsFactory::create(GraphicsAPI::OpenGL, &window);

    while window.is_running() {
        unsafe {
            Render.clear_color(0.0, 1.0, 1.0, 0.0);
            Render.clear();
        }

        window.poll_event();
    }
}