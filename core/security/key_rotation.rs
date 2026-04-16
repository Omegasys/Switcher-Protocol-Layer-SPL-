use std::collections::HashMap;

pub struct KeyRotation {
    pub keys: HashMap<String, u64>,
}

impl KeyRotation {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
        }
    }

    pub fn set_key(&mut self, id: &str, key: u64) {
        self.keys.insert(id.to_string(), key);
    }

    pub fn rotate(&mut self, id: &str) {
        if let Some(k) = self.keys.get_mut(id) {
            *k = k.rotate_left(7) ^ 0xBEEF;
        }
    }

    pub fn get_key(&self, id: &str) -> Option<u64> {
        self.keys.get(id).copied()
    }
}
