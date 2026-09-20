pub struct WindowDescriptor {
    pub title: String,
    pub width: u32,
    pub height: u32
}

impl WindowDescriptor {
    pub fn new(title: impl Into<String>, width: u32, height: u32) -> Self {
        Self {
            title: title.into(),
            width, height
        }
    }
}