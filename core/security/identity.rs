use std::collections::HashMap;

#[derive(Clone)]
pub struct Identity {
    pub node_id: String,
    pub public_key: u64,
    pub trust_score: f64,
}

pub struct IdentityManager {
    pub identities: HashMap<String, Identity>,
}

impl IdentityManager {
    pub fn new() -> Self {
        Self {
            identities: HashMap::new(),
        }
    }

    pub fn register(&mut self, node_id: &str, key: u64) {
        self.identities.insert(
            node_id.to_string(),
            Identity {
                node_id: node_id.to_string(),
                public_key: key,
                trust_score: 1.0,
            },
        );
    }

    pub fn update_trust(&mut self, node_id: &str, delta: f64) {
        if let Some(id) = self.identities.get_mut(node_id) {
            id.trust_score = (id.trust_score + delta).clamp(0.0, 1.0);
        }
    }

    pub fn get(&self, node_id: &str) -> Option<&Identity> {
        self.identities.get(node_id)
    }
}
