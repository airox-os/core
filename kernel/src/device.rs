pub struct DeviceManager;
impl DeviceManager {
    pub fn new() -> Self {
        crate::log("[DeviceManager] Initialized");
        DeviceManager
    }
    pub fn register_device(&self, device: &str) {
        crate::log(&format!("[DeviceManager] Device registered: {}", device));
    }
}
