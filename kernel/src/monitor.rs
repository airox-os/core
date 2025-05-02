pub struct Monitor;
impl Monitor {
    pub fn new() -> Self {
        crate::log("[Monitor] Initialized");
        Monitor
    }
    pub fn log_resource(&self, resource: &str, value: &str) {
        crate::log(&format!("[Monitor] {}: {}", resource, value));
    }
    pub fn health_check(&self) {
        crate::log("[Monitor] Health check OK");
    }
}
