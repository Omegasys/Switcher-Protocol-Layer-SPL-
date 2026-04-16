use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct VirtualNetwork {
    pub vni: u32,
    pub nodes: Vec<String>,
}

pub struct VirtualNetworkManager {
    pub networks: HashMap<u32, VirtualNetwork>,
}

impl VirtualNetworkManager {
    pub fn new() -> Self {
        Self {
            networks: HashMap::new(),
        }
    }

    pub fn create_network(&mut self, vni: u32) {
        self.networks.insert(
            vni,
            VirtualNetwork {
                vni,
                nodes: Vec::new(),
            },
        );
    }

    pub fn add_node(&mut self, vni: u32, node: &str) {
        if let Some(net) = self.networks.get_mut(&vni) {
            net.nodes.push(node.to_string());
        }
    }

    pub fn remove_node(&mut self, vni: u32, node: &str) {
        if let Some(net) = self.networks.get_mut(&vni) {
            net.nodes.retain(|n| n != node);
        }
    }

    pub fn get_network(&self, vni: u32) -> Option<&VirtualNetwork> {
        self.networks.get(&vni)
    }
}
