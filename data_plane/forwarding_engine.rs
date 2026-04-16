use std::collections::HashMap;

use crate::data_plane::packet_parser::ParsedPacket;

pub struct ForwardingEngine {
    pub routing_table: HashMap<String, String>, // dst -> next hop
}

impl ForwardingEngine {
    pub fn new() -> Self {
        Self {
            routing_table: HashMap::new(),
        }
    }

    pub fn install_route(&mut self, dst: &str, next_hop: &str) {
        self.routing_table
            .insert(dst.to_string(), next_hop.to_string());
    }

    pub fn forward(&self, packet: ParsedPacket) -> Option<String> {
        self.routing_table.get(&packet.dst).cloned()
    }
}
