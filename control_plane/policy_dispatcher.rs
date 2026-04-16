use std::collections::HashMap;

use crate::control_plane::intent_api::{Intent, IntentType};
use crate::controller::NodeState;

pub struct PolicyDispatcher;

impl PolicyDispatcher {
    pub fn new() -> Self {
        Self
    }

    pub fn dispatch(
        &self,
        intent: &Intent,
        nodes: &mut HashMap<String, NodeState>,
    ) {
        match intent.intent_type {
            IntentType::OptimizeLatency => {
                for node in nodes.values_mut() {
                    node.load *= 0.9;
                }
            }

            IntentType::ReduceCongestion => {
                for node in nodes.values_mut() {
                    node.load *= 0.8;
                }
            }

            IntentType::IncreaseReliability => {
                for node in nodes.values_mut() {
                    node.active = true;
                }
            }

            IntentType::MinimizeCost => {
                for node in nodes.values_mut() {
                    node.load *= 0.95;
                }
            }
        }
    }
}
