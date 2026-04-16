use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct EmulatedNode {
    pub id: String,
    pub latency_ms: f64,
    pub loss_rate: f64,
}

pub struct NetworkEmulator {
    pub nodes: HashMap<String, EmulatedNode>,
    pub links: HashMap<String, Vec<String>>,
}

impl NetworkEmulator {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            links: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, id: &str, latency: f64, loss: f64) {
        self.nodes.insert(
            id.to_string(),
            EmulatedNode {
                id: id.to_string(),
                latency_ms: latency,
                loss_rate: loss,
            },
        );
    }

    pub fn connect(&mut self, a: &str, b: &str) {
        self.links.entry(a.to_string()).or_default().push(b.to_string());
    }

    pub fn simulate_delay(&self, node: &str) -> f64 {
        self.nodes.get(node).map(|n| n.latency_ms).unwrap_or(1.0)
    }
}
