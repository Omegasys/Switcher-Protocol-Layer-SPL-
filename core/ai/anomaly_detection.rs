use std::collections::VecDeque;

pub struct AnomalyDetector {
    pub baseline: f64,
    pub threshold: f64,
    pub history: VecDeque<f64>,
}

impl AnomalyDetector {
    pub fn new(baseline: f64, threshold: f64) -> Self {
        Self {
            baseline,
            threshold,
            history: VecDeque::new(),
        }
    }

    pub fn observe(&mut self, value: f64) {
        self.history.push_back(value);
        if self.history.len() > 50 {
            self.history.pop_front();
        }
    }

    pub fn is_anomaly(&self, value: f64) -> bool {
        (value - self.baseline).abs() > self.threshold
    }

    pub fn rolling_anomaly_score(&self) -> f64 {
        if self.history.is_empty() {
            return 0.0;
        }

        let mean: f64 = self.history.iter().sum::<f64>() / self.history.len() as f64;

        let variance: f64 = self
            .history
            .iter()
            .map(|v| (v - mean).powi(2))
            .sum::<f64>()
            / self.history.len() as f64;

        variance.sqrt()
    }
}
