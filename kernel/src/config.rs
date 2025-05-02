pub struct Config;
impl Config {
    pub fn new() -> Self {
        crate::log("[Config] Initialized");
        Config
    }
    pub fn set(&self, key: &str, value: &str) {
        crate::log(&format!("[Config] Set {} = {}", key, value));
    }
    pub fn get(&self, key: &str) {
        crate::log(&format!("[Config] Get {}", key));
    }
}
