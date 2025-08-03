use worker::*;
use serde_json::Value;
use std::collections::HashMap;
use chrono::Utc;

/// Monitoring metrics
pub struct Metrics {
    request_count: HashMap<String, u64>,
    error_count: HashMap<String, u64>,
    response_times: Vec<f64>,
    cache_hits: u64,
    cache_misses: u64,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            request_count: HashMap::new(),
            error_count: HashMap::new(),
            response_times: Vec::new(),
            cache_hits: 0,
            cache_misses: 0,
        }
    }

    /// Record request start
    pub fn record_request_start(&mut self, request_id: &str) {
        let counter = self.request_count.entry(request_id.to_string()).or_insert(0);
        *counter += 1;
        console_log!("Request started: {}", request_id);
    }

    /// Record request completion
    pub fn record_request_complete(&mut self, request_id: &str, status_code: u16) {
        console_log!("Request completed: {} with status: {}", request_id, status_code);
    }

    /// Record error
    pub fn record_error(&mut self, request_id: &str, error_type: &str) {
        let counter = self.error_count.entry(error_type.to_string()).or_insert(0);
        *counter += 1;
        console_log!("Error recorded for {}: {}", request_id, error_type);
    }

    /// Record response time
    pub fn record_response_time(&mut self, request_id: &str, time_ms: f64) {
        self.response_times.push(time_ms);
        // Keep last 1000 response times
        if self.response_times.len() > 1000 {
            self.response_times.remove(0);
        }
        console_log!("Response time for {}: {}ms", request_id, time_ms);
    }

    /// Record cache hit
    pub fn record_cache_hit(&mut self, request_id: &str) {
        self.cache_hits += 1;
        console_log!("Cache hit for request: {}", request_id);
    }

    /// Record cache miss
    pub fn record_cache_miss(&mut self, request_id: &str) {
        self.cache_misses += 1;
        console_log!("Cache miss for request: {}", request_id);
    }

    /// Get statistics
    pub async fn get_stats(&self) -> Value {
        let avg_response_time = if self.response_times.is_empty() {
            0.0
        } else {
            self.response_times.iter().sum::<f64>() / self.response_times.len() as f64
        };

        let total_requests: u64 = self.request_count.values().sum();
        let total_errors: u64 = self.error_count.values().sum();
        let error_rate = if total_requests > 0 {
            (total_errors as f64 / total_requests as f64) * 100.0
        } else {
            0.0
        };

        let cache_hit_rate = if self.cache_hits + self.cache_misses > 0 {
            (self.cache_hits as f64 / (self.cache_hits + self.cache_misses) as f64) * 100.0
        } else {
            0.0
        };

        serde_json::json!({
            "total_requests": total_requests,
            "total_errors": total_errors,
            "error_rate": format!("{:.2}%", error_rate),
            "average_response_time": format!("{:.2}ms", avg_response_time),
            "cache_hits": self.cache_hits,
            "cache_misses": self.cache_misses,
            "cache_hit_rate": format!("{:.2}%", cache_hit_rate),
            "timestamp": Utc::now().to_rfc3339()
        })
    }

    /// Reset statistics
    pub fn reset(&mut self) {
        self.request_count.clear();
        self.error_count.clear();
        self.response_times.clear();
        self.cache_hits = 0;
        self.cache_misses = 0;
    }
}
