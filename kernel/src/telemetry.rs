pub struct Telemetry;
impl Telemetry {
    pub fn new() -> Self {
        crate::log("[Telemetry] Initialized");
        Telemetry
    }
}
