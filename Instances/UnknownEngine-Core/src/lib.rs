#[warn(nonstandard_style)]
pub struct Engine {
    running: bool,
}

impl Engine {
    pub fn new() -> Self { Self { running: false, } }

    pub fn start(&mut self) { self.running = true; }

    pub fn stop(&mut self) { self.running = false; }

    pub fn is_running(&self) -> bool { self.running }
}