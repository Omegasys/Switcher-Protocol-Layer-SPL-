use std::collections::VecDeque;

pub struct TrafficPredictor {
    pub history: VecDeque<f64>,
    pub window: usize,
}

impl TrafficPredictor {
    pub fn new(window: usize) -> Self {
        Self {
            history: VecDeque::new(),
            window,
        }
    }

    pub fn observe(&mut self, value: f64) {
        if self.history.len() == self.window {
            self.history.pop_front();
        }
        self.history.push_back(value);
    }

    pub fn predict_next(&self) -> f64 {
        if self.history.is_empty() {
            return 0.0;
        }

        let sum: f64 = self.history.iter().sum();
        sum / self.history.len() as f64
    }

    pub fn trend(&self) -> f64 {
        if self.history.len() < 2 {
            return 0.0;
        }

        let first = self.history.front().unwrap();
        let last = self.history.back().unwrap();
        last - first
    }
}
