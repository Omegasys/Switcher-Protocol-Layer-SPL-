pub struct HardwareAbstractionLayer {
    pub name: String,
    pub capacity_gbps: f64,
    pub active: bool,
}

impl HardwareAbstractionLayer {
    pub fn new(name: &str, capacity_gbps: f64) -> Self {
        Self {
            name: name.to_string(),
            capacity_gbps,
            active: true,
        }
    }

    pub fn send_packet(&self, packet_size_bytes: usize) -> bool {
        if !self.active {
            return false;
        }

        let load_estimate = (packet_size_bytes as f64) / 1_000_000.0;

        load_estimate < self.capacity_gbps
    }

    pub fn disable(&mut self) {
        self.active = false;
    }

    pub fn enable(&mut self) {
        self.active = true;
    }
}
