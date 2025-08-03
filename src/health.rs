use worker::*;
use crate::config::ProxyConfig;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Health checker
pub struct HealthChecker {
    unhealthy_backends: HashMap<String, DateTime<Utc>>,
    config: ProxyConfig,
}

impl HealthChecker {
    pub fn new(config: &ProxyConfig) -> Self {
        Self {
            unhealthy_backends: HashMap::new(),
            config: config.clone(),
        }
    }

    /// Check if backend is healthy
    pub async fn is_healthy(&self, backend: &str) -> bool {
        if !self.config.health_check_enabled {
            return true;
        }

        // Check if it's in the unhealthy list
        if let Some(unhealthy_time) = self.unhealthy_backends.get(backend) {
            let recovery_time = *unhealthy_time + chrono::Duration::seconds(self.config.health_check_interval as i64);
            if Utc::now() < recovery_time {
                return false;
            }
        }

        true
    }

    /// Mark backend as unhealthy
    pub async fn mark_unhealthy(&mut self, backend: &str) {
        console_log!("Marking backend as unhealthy: {}", backend);
        self.unhealthy_backends.insert(backend.to_string(), Utc::now());
    }

    /// Mark backend as healthy
    pub async fn mark_healthy(&mut self, backend: &str) {
        if self.unhealthy_backends.remove(backend).is_some() {
            console_log!("Marking backend as healthy: {}", backend);
        }
    }

    /// Perform health check
    pub async fn perform_health_check(&mut self, backend: &str) -> bool {
        if !self.config.health_check_enabled {
            return true;
        }

        let health_path = "/health"; // Default health check path
        let check_url = format!("{}{}", backend, health_path);

        console_log!("Performing health check for: {}", check_url);

        let request = match Request::new(&check_url, Method::Get) {
            Ok(req) => req,
            Err(_) => return false,
        };

        match Fetch::Request(request).send().await {
            Ok(response) => {
                let is_healthy = response.status_code() >= 200 && response.status_code() < 300;
                if is_healthy {
                    self.mark_healthy(backend).await;
                } else {
                    self.mark_unhealthy(backend).await;
                }
                is_healthy
            }
            Err(_) => {
                self.mark_unhealthy(backend).await;
                false
            }
        }
    }

    /// Get all healthy backends
    pub async fn get_healthy_backends(&self) -> Vec<String> {
        let mut healthy = Vec::new();
        for backend in &self.config.backends {
            if self.is_healthy(backend).await {
                healthy.push(backend.clone());
            }
        }
        healthy
    }

    /// Get backend health status
    pub async fn get_backend_status(&self) -> HashMap<String, bool> {
        let mut status = HashMap::new();
        for backend in &self.config.backends {
            status.insert(backend.clone(), self.is_healthy(backend).await);
        }
        status
    }
}
