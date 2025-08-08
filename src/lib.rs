use chrono::Utc;
use regex::Regex;
use uuid::Uuid;
use worker::*;

mod cache;
mod config;
mod health;
mod load_balancer;
mod middleware;
mod monitoring;
mod utils;

use cache::CacheManager;
use config::ProxyConfig;
use health::HealthChecker;
use load_balancer::{LoadBalancer, LoadBalancerStrategy};
use middleware::{apply_request_middleware, apply_response_middleware};
use monitoring::Metrics;

/// Main structure for the reverse proxy
pub struct ReverseProxy {
    config: ProxyConfig,
    load_balancer: LoadBalancer,
    health_checker: HealthChecker,
    metrics: Metrics,
    cache_manager: CacheManager,
}

impl ReverseProxy {
    /// Create reverse proxy instance from environment variables
    pub fn from_env(env: &Env) -> Result<Self> {
        let config = ProxyConfig::from_env(env)?;
        let strategy = LoadBalancerStrategy::from(config.load_balancer_strategy.as_str());
        let load_balancer = LoadBalancer::with_strategy(&config.backends, strategy);
        let health_checker = HealthChecker::new(&config);
        let metrics = Metrics::new();
        let cache_manager = CacheManager::new(&config);

        Ok(Self {
            config,
            load_balancer,
            health_checker,
            metrics,
            cache_manager,
        })
    }

    /// Handle incoming requests
    pub async fn handle_request(
        &mut self,
        mut req: Request,
        env: &Env,
        _ctx: &Context,
    ) -> Result<Response> {
        let request_id = Uuid::new_v4().to_string();
        let start_time = js_sys::Date::now();

        // Record request start
        self.metrics.record_request_start(&request_id);

        // Handle CORS preflight requests
        if req.method() == Method::Options {
            return self.handle_cors_preflight();
        }

        // Apply request middleware
        req = apply_request_middleware(req, &self.config)?;

        // Check for URL path proxy pattern (e.g., /https://example.com/path)
        let (target_url, is_url_proxy) = if let Some(url) =
            self.extract_target_url_from_path(&req)?
        {
            (url, true)
        } else {
            // Check cache for normal proxy requests
            if let Some(cached_response) = self.cache_manager.get_cached_response(&req, env).await?
            {
                self.metrics.record_cache_hit(&request_id);
                return Ok(cached_response);
            }

            // Get healthy backend for load-balanced proxy
            let backend = match self.load_balancer.get_backend(&self.health_checker).await {
                Some(backend) => backend,
                None => {
                    self.metrics.record_error(&request_id, "no_healthy_backend");
                    return Response::error("No healthy backends available", 503);
                }
            };

            // Build target URL using configured backend
            (self.build_target_url(&req, &backend)?, false)
        };

        console_log!(
            "Proxying request {} to: {} (URL proxy: {})",
            request_id,
            target_url,
            is_url_proxy
        );

        // Create proxy request
        let proxy_req = self.create_proxy_request(req, &target_url).await?;

        // Send request to backend
        let response = match Fetch::Request(proxy_req).send().await {
            Ok(response) => response,
            Err(e) => {
                self.metrics.record_error(&request_id, "backend_error");
                // Only mark backend unhealthy for load-balanced requests
                if !is_url_proxy {
                    // Extract backend URL for health check marking
                    let backend_base = target_url.split('/').take(3).collect::<Vec<_>>().join("/");
                    self.health_checker.mark_unhealthy(&backend_base).await;
                }
                console_log!("Backend error for {}: {:?}", request_id, e);
                return Response::error("Backend unavailable", 502);
            }
        };

        // Handle redirects for URL proxy mode
        let processed_response = if is_url_proxy && self.is_redirect_response(&response) {
            self.handle_redirect_response(response, &target_url).await?
        } else {
            response
        };

        // Record backend response time
        let response_time = js_sys::Date::now() - start_time;
        self.metrics
            .record_response_time(&request_id, response_time);

        // Apply response middleware and add CORS headers
        let mut final_response = apply_response_middleware(processed_response, &self.config)?;
        self.add_cors_headers(&mut final_response)?;

        // Record request completion
        self.metrics
            .record_request_complete(&request_id, final_response.status_code());

        // Cache response (if applicable)
        if self.should_cache_response(&final_response) {
            // Note: Caching consumes response, so we need to clone or redesign
            // Simplified handling here, can be improved in production
            console_log!("Response should be cached");
        }

        Ok(final_response)
    }

    /// Build target URL
    fn build_target_url(&self, req: &Request, backend: &str) -> Result<String> {
        let url = req.url()?;
        let path = url.path();
        let query = url.query();

        // Apply path rewrite rules
        let rewritten_path = self.apply_path_rewrite(path);

        let target_url = if let Some(q) = query {
            format!("{backend}{rewritten_path}?{q}")
        } else {
            format!("{backend}{rewritten_path}")
        };

        Ok(target_url)
    }

    /// Apply path rewrite rules
    fn apply_path_rewrite(&self, path: &str) -> String {
        for rule in &self.config.path_rewrite_rules {
            if let Ok(regex) = Regex::new(&rule.pattern) {
                if regex.is_match(path) {
                    return regex.replace(path, &rule.replacement).to_string();
                }
            }
        }
        path.to_string()
    }

    /// Create proxy request
    async fn create_proxy_request(&self, mut req: Request, target_url: &str) -> Result<Request> {
        let headers = req.headers().clone();

        // Add proxy-related headers
        if let Some(cf_ip) = headers.get("CF-Connecting-IP")? {
            headers.set("X-Forwarded-For", &cf_ip)?;
        }

        let url_str = req.url()?.to_string();
        let protocol = if url_str.starts_with("https:") {
            "https"
        } else {
            "http"
        };
        headers.set("X-Forwarded-Proto", protocol)?;

        if let Some(host) = headers.get("Host")? {
            headers.set("X-Forwarded-Host", &host)?;
        }

        // Remove headers that might cause issues
        headers.delete("Host")?;
        headers.delete("Origin")?;

        // Apply custom headers
        for (key, value) in &self.config.custom_headers {
            headers.set(key, value)?;
        }

        let mut init = RequestInit::new();
        init.with_method(req.method()).with_headers(headers);

        // Copy request body if present
        if req.method() != Method::Get && req.method() != Method::Head {
            let body_bytes = req.bytes().await?;
            init.with_body(Some(body_bytes.into()));
        }

        Request::new_with_init(target_url, &init)
    }

    /// Determine if response should be cached
    fn should_cache_response(&self, response: &Response) -> bool {
        if !self.config.cache_enabled {
            return false;
        }

        let status = response.status_code();
        if !(200..300).contains(&status) {
            return false;
        }

        // Check cache control headers
        if let Ok(Some(cache_control)) = response.headers().get("Cache-Control") {
            if cache_control.contains("no-cache") || cache_control.contains("no-store") {
                return false;
            }
        }

        true
    }

    /// Health check endpoint
    pub async fn health_check(&self) -> Result<Response> {
        let healthy_backends = self.health_checker.get_healthy_backends().await;
        let total_backends = self.config.backends.len();

        let health_status = serde_json::json!({
            "status": if healthy_backends.is_empty() { "unhealthy" } else { "healthy" },
            "healthy_backends": healthy_backends.len(),
            "total_backends": total_backends,
            "backends": self.config.backends,
            "timestamp": Utc::now().to_rfc3339()
        });

        Response::from_json(&health_status)
    }

    /// Get proxy statistics
    pub async fn get_stats(&self) -> Result<Response> {
        let stats = self.metrics.get_stats().await;
        Response::from_json(&stats)
    }

    /// Extract target URL from path (e.g., /https://example.com/path)
    fn extract_target_url_from_path(&self, req: &Request) -> Result<Option<String>> {
        let url = req.url()?;
        let path = url.path();

        // Check if path starts with /http:// or /https://
        if let Some(target_start) = path.strip_prefix("/") {
            if target_start.starts_with("http://") || target_start.starts_with("https://") {
                // Parse the embedded URL
                if let Ok(embedded_url) = url::Url::parse(target_start) {
                    let mut target_url = embedded_url.to_string();

                    // Add query parameters from the original request if they exist
                    if let Some(query) = url.query() {
                        let separator = if embedded_url.query().is_some() {
                            "&"
                        } else {
                            "?"
                        };
                        target_url = format!("{target_url}{separator}{query}");
                    }

                    return Ok(Some(target_url));
                }
            }
        }

        Ok(None)
    }

    /// Check if response is a redirect
    fn is_redirect_response(&self, response: &Response) -> bool {
        let status = response.status_code();
        (300..400).contains(&status)
    }

    /// Handle redirect response by modifying location header
    async fn handle_redirect_response(
        &self,
        response: Response,
        original_target: &str,
    ) -> Result<Response> {
        if let Ok(Some(location)) = response.headers().get("Location") {
            // If the location is relative, make it absolute
            let new_location = if location.starts_with("/") {
                if let Ok(target_url) = url::Url::parse(original_target) {
                    format!(
                        "{}://{}{}",
                        target_url.scheme(),
                        target_url.host_str().unwrap_or(""),
                        location
                    )
                } else {
                    location
                }
            } else if location.starts_with("http://") || location.starts_with("https://") {
                // Absolute URL - we might want to proxy this through our worker too
                location
            } else {
                // Relative to current path
                if let Ok(target_url) = url::Url::parse(original_target) {
                    if let Ok(base_url) = target_url.join(&location) {
                        base_url.to_string()
                    } else {
                        location
                    }
                } else {
                    location
                }
            };

            // Update the location header
            response.headers().set("Location", &new_location)?;
        }

        Ok(response)
    }

    /// Add CORS headers to response
    fn add_cors_headers(&self, response: &mut Response) -> Result<()> {
        let headers = response.headers();
        headers.set("Access-Control-Allow-Origin", "*")?;
        headers.set(
            "Access-Control-Allow-Methods",
            "GET, POST, PUT, DELETE, OPTIONS, HEAD, PATCH",
        )?;
        headers.set("Access-Control-Allow-Headers", "Content-Type, Authorization, X-Requested-With, Accept, Origin, User-Agent, DNT, Cache-Control, X-Mx-ReqToken, Keep-Alive, X-Requested-With, If-Modified-Since")?;
        headers.set("Access-Control-Max-Age", "86400")?;
        headers.set("Access-Control-Allow-Credentials", "true")?;
        Ok(())
    }

    /// Handle CORS preflight requests
    fn handle_cors_preflight(&self) -> Result<Response> {
        let mut response = Response::empty()?;
        self.add_cors_headers(&mut response)?;
        Ok(response)
    }
}

/// Main entry point
#[event(fetch)]
pub async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    // Create proxy instance
    let mut proxy = match ReverseProxy::from_env(&env) {
        Ok(proxy) => proxy,
        Err(e) => {
            console_log!("Failed to initialize proxy: {:?}", e);
            return Response::error("Proxy configuration error", 500);
        }
    };

    let url = req.url()?;
    let path = url.path();

    // Handle management endpoints
    match path {
        "/_proxy/health" => proxy.health_check().await,
        "/_proxy/stats" => proxy.get_stats().await,
        _ => proxy.handle_request(req, &env, &ctx).await,
    }
}
