use worker::*;

use crate::config::ProxyConfig;

/// Cache manager
pub struct CacheManager {
    config: ProxyConfig,
}

impl CacheManager {
    pub fn new(config: &ProxyConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }

    /// Get cached response
    pub async fn get_cached_response(&self, req: &Request, env: &Env) -> Result<Option<Response>> {
        if !self.config.cache_enabled {
            return Ok(None);
        }

        let cache_key = self.generate_cache_key(req)?;

        // Try to get cache from KV storage
        if let Ok(kv) = env.kv("PROXY_KV") {
            if let Ok(Some(cached_data)) = kv.get(&cache_key).text().await {
                console_log!("Cache hit for key: {}", cache_key);
                // Here should deserialize response data
                // Simplified implementation: return text response
                return Ok(Some(Response::ok(cached_data)?));
            }
        }

        console_log!("Cache miss for key: {}", cache_key);
        Ok(None)
    }

    /// Cache response
    #[allow(dead_code)]
    pub async fn cache_response(
        &self,
        mut response: Response,
        env: &Env,
        _ctx: &Context,
    ) -> Result<()> {
        if !self.config.cache_enabled {
            return Ok(());
        }

        // Check if response is cacheable
        if !self.is_cacheable(&response) {
            return Ok(());
        }

        let cache_key = self.generate_cache_key_from_response(&response)?;

        // Get response content
        let response_text = response.text().await?;

        // Store to KV (simplified implementation)
        if let Ok(kv) = env.kv("PROXY_KV") {
            let expiration_ttl = self.config.cache_ttl;

            // Simplified cache implementation
            if let Err(e) = kv
                .put(&cache_key, &response_text)?
                .expiration_ttl(expiration_ttl)
                .execute()
                .await
            {
                console_log!("Failed to cache response: {:?}", e);
            } else {
                console_log!(
                    "Cached response with key: {} (TTL: {}s)",
                    cache_key,
                    expiration_ttl
                );
            }
        }

        Ok(())
    }

    /// Generate cache key
    fn generate_cache_key(&self, req: &Request) -> Result<String> {
        let url = req.url()?;
        let path = url.path();
        let query = url.query().unwrap_or("");
        let method = req.method().to_string();

        // Simple cache key generation, can be made more complex as needed
        let cache_key = format!("proxy:{method}:{path}:{query}");

        // Use SHA-256 hash to ensure reasonable key length
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(cache_key.as_bytes());
        let hash = hasher.finalize();
        Ok(hex::encode(hash))
    }

    /// Generate cache key from response (simplified implementation)
    #[allow(dead_code)]
    fn generate_cache_key_from_response(&self, _response: &Response) -> Result<String> {
        // Here should generate key based on original request, simplified implementation
        Ok(uuid::Uuid::new_v4().to_string())
    }

    /// Check if response is cacheable
    #[allow(dead_code)]
    fn is_cacheable(&self, response: &Response) -> bool {
        let status = response.status_code();

        // Only cache successful responses
        if !(200..300).contains(&status) {
            return false;
        }

        // Check Cache-Control header
        if let Ok(Some(cache_control)) = response.headers().get("Cache-Control") {
            if cache_control.contains("no-cache")
                || cache_control.contains("no-store")
                || cache_control.contains("private")
            {
                return false;
            }
        }

        // Check Vary header, don't cache if too variable
        if let Ok(Some(vary)) = response.headers().get("Vary") {
            if vary.to_lowercase().contains("*") {
                return false;
            }
        }

        true
    }

    /// Clear cache
    #[allow(dead_code)]
    pub async fn clear_cache(&self, _env: &Env) -> Result<()> {
        console_log!("Clearing proxy cache");
        // In actual implementation, need to iterate and delete all keys with specific prefix
        // This is a simplified implementation
        Ok(())
    }

    /// Get cache statistics
    #[allow(dead_code)]
    pub async fn get_cache_stats(&self, _env: &Env) -> Result<serde_json::Value> {
        // Simplified cache statistics implementation
        Ok(serde_json::json!({
            "cache_enabled": self.config.cache_enabled,
            "cache_ttl": self.config.cache_ttl,
            "cache_type": "KV Store"
        }))
    }
}
