pub struct DeviceManager;
impl DeviceManager {
    pub fn new() -> Self {
        crate::log("[DeviceManager] Initialized");
        DeviceManager
    }
    pub fn register_device(&self, _device: &str) {
        crate::log("[DeviceManager] Device registered.");
    }
}
