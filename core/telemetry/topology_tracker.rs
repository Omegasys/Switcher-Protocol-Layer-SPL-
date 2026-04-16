use std::collections::{HashMap, HashSet};

pub struct TopologyTracker {
    pub nodes: HashSet<String>,
    pub links: HashMap<String, Vec<String>>,
}

impl TopologyTracker {
    pub fn new() -> Self {
        Self {
            nodes: HashSet::new(),
            links: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: &str) {
        self.nodes.insert(node.to_string());
    }

    pub fn add_link(&mut self, from: &str, to: &str) {
        self.links
            .entry(from.to_string())
            .or_default()
            .push(to.to_string());
    }

    pub fn remove_node(&mut self, node: &str) {
        self.nodes.remove(node);
        self.links.remove(node);
        for (_, neighbors) in self.links.iter_mut() {
            neighbors.retain(|n| n != node);
        }
    }

    pub fn neighbors(&self, node: &str) -> Option<&Vec<String>> {
        self.links.get(node)
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
}
