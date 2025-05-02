pub struct Logger;
impl Logger {
    pub fn new() -> Self {
        Logger
    }
    pub fn log(&self, message: &str) {
        crate::log(&format!("[LOG] {}", message));
    }
    pub fn error(&self, message: &str) {
        crate::log(&format!("[ERROR] {}", message));
    }
}
