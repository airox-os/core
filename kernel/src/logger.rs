pub struct Logger;
impl Logger {
    pub fn new() -> Self {
        Logger
    }
    pub fn log(&self, _message: &str) {
        crate::log("[LOG] Message");
    }
    pub fn error(&self, _message: &str) {
        crate::log("[ERROR] Message");
    }
}
