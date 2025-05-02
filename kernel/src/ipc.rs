pub struct IPC;
impl IPC {
    pub fn new() -> Self {
        crate::log("[IPC] Initialized");
        IPC
    }
    pub fn send(&self, channel: &str, message: &str) {
        crate::log(&format!("[IPC] Sent on {}: {}", channel, message));
    }
    pub fn receive(&self, channel: &str) {
        crate::log(&format!("[IPC] Receive on {}", channel));
    }
}
