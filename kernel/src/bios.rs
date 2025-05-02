pub struct BIOS;
impl BIOS {
    pub fn new() -> Self {
        crate::log("[BIOS] Initialized");
        BIOS
    }
}
