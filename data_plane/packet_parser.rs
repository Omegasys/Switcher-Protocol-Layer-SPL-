#[derive(Debug, Clone)]
pub struct RawPacket {
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct ParsedPacket {
    pub src: String,
    pub dst: String,
    pub payload: Vec<u8>,
}

pub struct PacketParser;

impl PacketParser {
    pub fn parse(packet: RawPacket) -> ParsedPacket {
        // Simplified mock parsing logic
        ParsedPacket {
            src: "node_a".to_string(),
            dst: "node_b".to_string(),
            payload: packet.data,
        }
    }
}
