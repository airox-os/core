pub struct IPC;
impl IPC {
    pub fn new() -> Self {
        crate::log("[IPC] Initialized");
        IPC
    }
    pub fn send(&self, _channel: &str, _message: &str) {
        crate::log("[IPC] Message sent.");
    }
    pub fn receive(&self, _channel: &str) {
        crate::log("[IPC] Message received.");
    }
}
