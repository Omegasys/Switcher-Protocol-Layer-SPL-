use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Tunnel {
    pub id: String,
    pub src: String,
    pub dst: String,
    pub encrypted: bool,
}

pub struct TunnelManager {
    pub tunnels: HashMap<String, Tunnel>,
}

impl TunnelManager {
    pub fn new() -> Self {
        Self {
            tunnels: HashMap::new(),
        }
    }

    pub fn create_tunnel(&mut self, id: &str, src: &str, dst: &str, encrypted: bool) {
        self.tunnels.insert(
            id.to_string(),
            Tunnel {
                id: id.to_string(),
                src: src.to_string(),
                dst: dst.to_string(),
                encrypted,
            },
        );
    }

    pub fn get_tunnel(&self, id: &str) -> Option<&Tunnel> {
        self.tunnels.get(id)
    }

    pub fn remove_tunnel(&mut self, id: &str) {
        self.tunnels.remove(id);
    }

    pub fn list_tunnels(&self) -> Vec<&Tunnel> {
        self.tunnels.values().collect()
    }
}
