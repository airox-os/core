pub struct Config;
impl Config {
    pub fn new() -> Self {
        crate::log("[Config] Initialized");
        Config
    }
    pub fn set(&self, _key: &str, _value: &str) {
        crate::log("[Config] Set value.");
    }
    pub fn get(&self, _key: &str) {
        crate::log("[Config] Get value.");
    }
}
