use rand::Rng;
use std::collections::HashMap;

pub struct TopologyGenerator;

impl TopologyGenerator {
    pub fn random_topology(nodes: usize) -> HashMap<String, Vec<String>> {
        let mut graph = HashMap::new();
        let mut rng = rand::thread_rng();

        let node_ids: Vec<String> = (0..nodes)
            .map(|i| format!("node_{}", i))
            .collect();

        for node in &node_ids {
            let mut neighbors = Vec::new();

            for other in &node_ids {
                if node != other && rng.gen_bool(0.3) {
                    neighbors.push(other.clone());
                }
            }

            graph.insert(node.clone(), neighbors);
        }

        graph
    }
}
