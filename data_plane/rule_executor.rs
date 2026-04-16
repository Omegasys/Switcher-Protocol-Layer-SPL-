use std::collections::HashMap;

use crate::data_plane::packet_parser::ParsedPacket;

#[derive(Clone)]
pub struct Rule {
    pub match_dst: Option<String>,
    pub action: String,
}

pub struct RuleExecutor {
    pub rules: Vec<Rule>,
}

impl RuleExecutor {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub fn execute(&self, packet: &ParsedPacket) -> Option<String> {
        for rule in &self.rules {
            if let Some(dst) = &rule.match_dst {
                if dst == &packet.dst {
                    return Some(rule.action.clone());
                }
            }
        }
        None
    }
}
