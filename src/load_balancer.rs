use std::sync::atomic::{AtomicUsize, Ordering};

use crate::health::HealthChecker;

/// Load balancer strategy
#[derive(Debug, Clone)]
pub enum LoadBalancerStrategy {
    RoundRobin,
    Random,
    LeastConnections,
    WeightedRoundRobin,
}

impl From<&str> for LoadBalancerStrategy {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "random" => LoadBalancerStrategy::Random,
            "least_connections" => LoadBalancerStrategy::LeastConnections,
            "weighted_round_robin" => LoadBalancerStrategy::WeightedRoundRobin,
            _ => LoadBalancerStrategy::RoundRobin,
        }
    }
}

/// Load balancer
pub struct LoadBalancer {
    #[allow(dead_code)]
    backends: Vec<String>,
    strategy: LoadBalancerStrategy,
    current_index: AtomicUsize,
}

impl LoadBalancer {
    #[allow(dead_code)]
    pub fn with_strategy(backends: &[String], strategy: LoadBalancerStrategy) -> Self {
        Self {
            backends: backends.to_vec(),
            strategy,
            current_index: AtomicUsize::new(0),
        }
    }

    /// Get next backend server
    pub async fn get_backend(&self, health_checker: &HealthChecker) -> Option<String> {
        let healthy_backends = health_checker.get_healthy_backends().await;
        if healthy_backends.is_empty() {
            return None;
        }

        match self.strategy {
            LoadBalancerStrategy::RoundRobin => self.round_robin_select(&healthy_backends),
            LoadBalancerStrategy::Random => self.random_select(&healthy_backends),
            LoadBalancerStrategy::LeastConnections => {
                self.least_connections_select(&healthy_backends)
            }
            LoadBalancerStrategy::WeightedRoundRobin => {
                self.weighted_round_robin_select(&healthy_backends)
            }
        }
    }

    fn round_robin_select(&self, backends: &[String]) -> Option<String> {
        if backends.is_empty() {
            return None;
        }

        let index = self.current_index.fetch_add(1, Ordering::Relaxed) % backends.len();
        Some(backends[index].clone())
    }

    fn random_select(&self, backends: &[String]) -> Option<String> {
        if backends.is_empty() {
            return None;
        }

        // Simple pseudo-random number generation (based on current time)
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as usize;
        let index = now % backends.len();
        Some(backends[index].clone())
    }

    fn least_connections_select(&self, backends: &[String]) -> Option<String> {
        // Simplified implementation: fallback to round robin
        // In actual implementation, need to track connection count for each backend
        self.round_robin_select(backends)
    }

    fn weighted_round_robin_select(&self, backends: &[String]) -> Option<String> {
        // Simplified implementation: fallback to round robin
        // In actual implementation, need to select based on weights
        self.round_robin_select(backends)
    }

    /// Get all backends
    #[allow(dead_code)]
    pub fn get_all_backends(&self) -> &[String] {
        &self.backends
    }

    /// Update backend list
    #[allow(dead_code)]
    pub fn update_backends(&mut self, backends: Vec<String>) {
        self.backends = backends;
        self.current_index.store(0, Ordering::Relaxed);
    }
}
