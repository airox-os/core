pub struct Network;
impl Network {
    pub fn new() -> Self {
        crate::log("[Network] Subsystem initialized");
        Network
    }
    pub fn send(&self, _address: &str, _data: &str) {
        crate::log("[Network] Sending data.");
    }
    pub fn receive(&self, _address: &str) {
        crate::log("[Network] Receiving data.");
    }
}
