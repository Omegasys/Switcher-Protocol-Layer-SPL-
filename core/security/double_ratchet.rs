pub struct DoubleRatchet {
    pub root_key: u64,
    pub chain_key: u64,
}

impl DoubleRatchet {
    pub fn new(seed: u64) -> Self {
        Self {
            root_key: seed,
            chain_key: seed ^ 0xA5A5A5A5,
        }
    }

    fn kdf(x: u64) -> u64 {
        x.rotate_left(13) ^ 0xDEADBEEF
    }

    pub fn ratchet_step(&mut self) {
        self.chain_key = Self::kdf(self.chain_key);
        self.root_key = Self::kdf(self.root_key ^ self.chain_key);
    }

    pub fn encrypt(&self, data: u64) -> u64 {
        data ^ self.chain_key
    }

    pub fn decrypt(&self, data: u64) -> u64 {
        data ^ self.chain_key
    }
}
