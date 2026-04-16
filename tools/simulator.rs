use std::collections::HashMap;

use crate::control_plane::controller::Controller;
use crate::data_plane::packet_parser::{RawPacket, PacketParser};
use crate::data_plane::forwarding_engine::ForwardingEngine;

pub struct Simulator {
    pub controller: Controller,
    pub forwarding_engine: ForwardingEngine,
}

impl Simulator {
    pub fn new(controller: Controller, forwarding_engine: ForwardingEngine) -> Self {
        Self {
            controller,
            forwarding_engine,
        }
    }

    pub fn step(&mut self, raw: RawPacket) {
        let packet = PacketParser::parse(raw);

        if let Some(next_hop) = self.forwarding_engine.forward(packet.clone()) {
            println!("Forwarded to: {}", next_hop);
        } else {
            println!("Packet dropped (no route)");
        }
    }
}
