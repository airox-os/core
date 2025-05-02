pub struct Network;
impl Network {
    pub fn new() -> Self {
        crate::log("[Network] Subsystem initialized");
        Network
    }
    pub fn send(&self, address: &str, data: &str) {
        crate::log(&format!("[Network] Sending to {}: {}", address, data));
    }
    pub fn receive(&self, address: &str) {
        crate::log(&format!("[Network] Receiving from {}", address));
    }
}
