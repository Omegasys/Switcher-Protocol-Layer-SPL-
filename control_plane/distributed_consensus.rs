use std::collections::HashMap;

pub struct DistributedConsensus {
    pub votes: HashMap<String, u32>,
    pub threshold: u32,
}

impl DistributedConsensus {
    pub fn new(threshold: u32) -> Self {
        Self {
            votes: HashMap::new(),
            threshold,
        }
    }

    pub fn vote(&mut self, proposal: &str) {
        let counter = self.votes.entry(proposal.to_string()).or_insert(0);
        *counter += 1;
    }

    pub fn is_accepted(&self, proposal: &str) -> bool {
        self.votes.get(proposal).copied().unwrap_or(0) >= self.threshold
    }

    pub fn reset(&mut self) {
        self.votes.clear();
    }
}
