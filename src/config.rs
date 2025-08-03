use serde::{Deserialize, Serialize};
use worker::*;

/// Path rewrite rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathRewriteRule {
    pub pattern: String,
    pub replacement: String,
}

/// Backend server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendConfig {
    pub url: String,
    pub weight: u32,
    pub health_check_path: Option<String>,
    pub timeout: Option<u64>,
}

/// Access control rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessRule {
    pub rule_type: String, // "allow" or "deny"
    pub pattern: String,   // IP, CIDR, or country code
}

/// Proxy configuration
#[derive(Debug, Clone)]
pub struct ProxyConfig {
    pub backends: Vec<String>,
    pub backend_configs: Vec<BackendConfig>,
    pub load_balancer_strategy: String,
    pub health_check_enabled: bool,
    pub health_check_interval: u64,
    #[allow(dead_code)]
    pub health_check_timeout: u64,
    pub cache_enabled: bool,
    pub cache_ttl: u64,
    pub path_rewrite_rules: Vec<PathRewriteRule>,
    pub custom_headers: std::collections::HashMap<String, String>,
    pub access_rules: Vec<AccessRule>,
    pub log_level: String,
    pub timeout: u64,
    pub retry_attempts: u32,
}

impl Default for ProxyConfig {
    fn default() -> Self {
        Self {
            backends: vec!["https://httpbin.org".to_string()],
            backend_configs: vec![],
            load_balancer_strategy: "round_robin".to_string(),
            health_check_enabled: true,
            health_check_interval: 30,
            health_check_timeout: 5,
            cache_enabled: false,
            cache_ttl: 300,
            path_rewrite_rules: vec![],
            custom_headers: std::collections::HashMap::new(),
            access_rules: vec![],
            log_level: "info".to_string(),
            timeout: 30,
            retry_attempts: 3,
        }
    }
}

impl ProxyConfig {
    /// Create configuration from environment variables
    pub fn from_env(env: &Env) -> Result<Self> {
        let mut config = Self::default();

        // Parse backend URL list
        if let Ok(backends_json) = env.var("BACKEND_URLS") {
            if let Ok(backends) = serde_json::from_str::<Vec<String>>(&backends_json.to_string()) {
                config.backends = backends;
            }
        }

        // Parse backend configurations
        if let Ok(backend_configs_json) = env.var("BACKEND_CONFIGS") {
            if let Ok(backend_configs) =
                serde_json::from_str::<Vec<BackendConfig>>(&backend_configs_json.to_string())
            {
                config.backend_configs = backend_configs;
            }
        }

        // Load balancer strategy
        if let Ok(strategy) = env.var("LOAD_BALANCER_STRATEGY") {
            config.load_balancer_strategy = strategy.to_string();
        }

        // Health check configuration
        if let Ok(enabled) = env.var("HEALTH_CHECK_ENABLED") {
            config.health_check_enabled = enabled.to_string().parse().unwrap_or(true);
        }

        if let Ok(interval) = env.var("HEALTH_CHECK_INTERVAL") {
            config.health_check_interval = interval.to_string().parse().unwrap_or(30);
        }

        // Cache configuration
        if let Ok(enabled) = env.var("CACHE_ENABLED") {
            config.cache_enabled = enabled.to_string().parse().unwrap_or(false);
        }

        if let Ok(ttl) = env.var("CACHE_TTL") {
            config.cache_ttl = ttl.to_string().parse().unwrap_or(300);
        }

        // Path rewrite rules
        if let Ok(rules_json) = env.var("PATH_REWRITE_RULES") {
            if let Ok(rules) = serde_json::from_str::<Vec<PathRewriteRule>>(&rules_json.to_string())
            {
                config.path_rewrite_rules = rules;
            }
        }

        // Custom headers
        if let Ok(headers_json) = env.var("CUSTOM_HEADERS") {
            if let Ok(headers) = serde_json::from_str::<std::collections::HashMap<String, String>>(
                &headers_json.to_string(),
            ) {
                config.custom_headers = headers;
            }
        }

        // Access control rules
        if let Ok(rules_json) = env.var("ACCESS_RULES") {
            if let Ok(rules) = serde_json::from_str::<Vec<AccessRule>>(&rules_json.to_string()) {
                config.access_rules = rules;
            }
        }

        // Log level
        if let Ok(log_level) = env.var("LOG_LEVEL") {
            config.log_level = log_level.to_string();
        }

        // Timeout configuration
        if let Ok(timeout) = env.var("TIMEOUT") {
            config.timeout = timeout.to_string().parse().unwrap_or(30);
        }

        // Retry attempts
        if let Ok(retry) = env.var("RETRY_ATTEMPTS") {
            config.retry_attempts = retry.to_string().parse().unwrap_or(3);
        }

        Ok(config)
    }
}
