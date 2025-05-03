pub struct UserManager;
impl UserManager {
    pub fn new() -> Self {
        crate::log("[UserManager] Initialized");
        UserManager
    }
    pub fn add_user(&self, _username: &str, _info: &str) {
        crate::log("[UserManager] User added.");
    }
    pub fn get_user(&self, _username: &str) {
        crate::log("[UserManager] Get user.");
    }
}
