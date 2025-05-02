pub struct UserManager;
impl UserManager {
    pub fn new() -> Self {
        crate::log("[UserManager] Initialized");
        UserManager
    }
    pub fn add_user(&self, username: &str, _info: &str) {
        crate::log(&format!("[UserManager] User added: {}", username));
    }
    pub fn get_user(&self, username: &str) {
        crate::log(&format!("[UserManager] Get user: {}", username));
    }
}
