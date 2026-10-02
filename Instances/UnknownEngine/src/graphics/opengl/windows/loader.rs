use crate::library::windows::WinLibrary;

struct WOpenGLL {
    opengl32: WinLibrary
}

impl WOpenGLL {
    fn new() -> Self {
        Self {
            opengl32: WinLibrary::load("opengl32.dll").expect("Failed to load opengl32.dll")
        }
    }
}
