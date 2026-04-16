use std::collections::HashMap;
use crate::switcher::mac_table::MacTable;
use crate::switcher::vlan_manager::VlanManager;

#[derive(Debug, Clone)]
pub struct EthernetFrame {
    pub src_mac: String,
    pub dst_mac: String,
    pub vlan_id: Option<u16>,
    pub payload: Vec<u8>,
}

pub struct L2SwitchEngine {
    pub mac_table: MacTable,
    pub vlan_manager: VlanManager,
}

impl L2SwitchEngine {
    pub fn new() -> Self {
        Self {
            mac_table: MacTable::new(),
            vlan_manager: VlanManager::new(),
        }
    }

    pub fn learn(&mut self, frame: &EthernetFrame, ingress_port: u32) {
        self.mac_table
            .learn(frame.src_mac.clone(), ingress_port);
    }

    pub fn forward(&self, frame: &EthernetFrame) -> Option<u32> {
        self.mac_table.lookup(&frame.dst_mac)
    }

    pub fn process_frame(&mut self, frame: EthernetFrame, ingress_port: u32) -> Option<u32> {
        self.learn(&frame, ingress_port);

        if let Some(vlan) = frame.vlan_id {
            if !self.vlan_manager.is_allowed(vlan, ingress_port) {
                return None;
            }
        }

        self.forward(&frame)
    }
}
