use std::collections::HashMap;

pub struct SessionCrypto {
    pub sessions: HashMap<String, u64>, // session_id -> key state
}

impl SessionCrypto {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    pub fn create_session(&mut self, session_id: &str) {
        self.sessions.insert(session_id.to_string(), 1);
    }

    pub fn encrypt(&self, session_id: &str, data: u64) -> Option<u64> {
        let key = self.sessions.get(session_id)?;
        Some(data ^ key)
    }

    pub fn decrypt(&self, session_id: &str, data: u64) -> Option<u64> {
        self.encrypt(session_id, data) // symmetric XOR stub
    }
}
