pub struct Shell;
impl Shell {
    pub fn new() -> Self {
        crate::log("[Shell] Initialized");
        Shell
    }
    pub fn execute(&self, command: &str) {
        crate::log(&format!("[Shell] Executing: {}", command));
    }
}
