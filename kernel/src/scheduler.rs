pub struct Scheduler;
impl Scheduler {
    pub fn new() -> Self {
        crate::log("[Scheduler] Initialized");
        Scheduler
    }
    pub fn schedule(&self, task: &str, time: &str) {
        crate::log(&format!("[Scheduler] Task scheduled: {} at {}", task, time));
    }
}
