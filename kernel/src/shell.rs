pub struct Shell;
impl Shell {
    pub fn new() -> Self {
        crate::log("[Shell] Initialized");
        Shell
    }
    pub fn execute(&self, _command: &str) {
        crate::log("[Shell] Executing command.");
    }
}
