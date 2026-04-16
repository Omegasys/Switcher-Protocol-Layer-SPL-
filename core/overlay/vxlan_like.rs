use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct VxlanPacket {
    pub vni: u32,              // virtual network identifier
    pub src: String,
    pub dst: String,
    pub payload: Vec<u8>,
}

pub struct VxlanLike {
    pub vni_table: HashMap<u32, String>, // VNI -> network name
}

impl VxlanLike {
    pub fn new() -> Self {
        Self {
            vni_table: HashMap::new(),
        }
    }

    pub fn register_network(&mut self, vni: u32, name: &str) {
        self.vni_table.insert(vni, name.to_string());
    }

    pub fn encapsulate(&self, vni: u32, src: &str, dst: &str, payload: Vec<u8>) -> VxlanPacket {
        VxlanPacket {
            vni,
            src: src.to_string(),
            dst: dst.to_string(),
            payload,
        }
    }

    pub fn decapsulate(&self, packet: VxlanPacket) -> Vec<u8> {
        packet.payload
    }

    pub fn resolve_network(&self, vni: u32) -> Option<&String> {
        self.vni_table.get(&vni)
    }
}
