# Cloudflare Workers Rust Reverse Proxy - AI Coding Instructions

## Project Overview
This is a production-ready reverse proxy built with Rust targeting Cloudflare Workers (WebAssembly). It provides load balancing, health checks, caching, access control, and URL path proxying with a modular architecture.

## Key Architecture Components

### Entry Point & Request Flow
- `src/lib.rs` contains the main `ReverseProxy` struct and `main()` entry point with `#[event(fetch)]`
- Two proxy modes: **URL path proxy** (`/https://example.com/path`) and **load-balanced backend proxy**
- Management endpoints: `/_proxy/health` and `/_proxy/stats`
- Request flow: middleware → load balancer OR URL extraction → backend request → response middleware → CORS

### Module Structure
- `config.rs`: Environment variable parsing into typed `ProxyConfig` struct with JSON deserialization
- `load_balancer.rs`: Thread-safe round-robin/random/least-connections with `AtomicUsize` counters
- `middleware.rs`: Access control (IP/country/user-agent), security headers, CORS handling
- `health.rs`: Backend health checking with automatic unhealthy backend marking
- `cache.rs`: KV-based response caching with TTL and cache control header respect
- `monitoring.rs`: Request metrics, response times, error tracking with UUID request IDs
- `utils.rs`: Utility functions for IP extraction, URL parsing, encoding

## Critical Development Patterns

### Configuration Management
```rust
// Environment variables are parsed as JSON strings in wrangler.toml
BACKEND_URLS = '["https://api1.com", "https://api2.com"]'
ACCESS_RULES = '[{"rule_type": "deny_country", "pattern": "CN"}]'
```
Always validate JSON parsing in `config.rs` with fallback defaults.

### WebAssembly Build Process
```bash
# Required for compilation
cargo install worker-build
worker-build --release  # Generates build/worker/shim.mjs
```
The `wrangler.toml` build command handles this automatically. Never edit generated files in `build/`.

### Request/Response Handling
- Use `RequestInit` for creating proxy requests, not direct `Request::new()`
- Always clone headers before modification: `let headers = req.headers().clone()`
- CORS headers must be added to every response, including errors
- Handle request body copying for non-GET/HEAD methods: `req.bytes().await?`

### Error Handling & Logging
- Use `console_log!()` macro for structured logging (not `println!`)
- Return proper HTTP status codes: 502 for backend errors, 503 for no healthy backends
- Record metrics for all request outcomes using request IDs
- Mark backends unhealthy on connection failures (load-balanced mode only)

## Development Workflow

### Local Development
```bash
wrangler dev  # Starts local server at localhost:8787
wrangler tail # Real-time log viewing
```

### Testing Proxy Modes
```bash
# URL path proxy
curl https://your-worker.dev/https://httpbin.org/json

# Load-balanced proxy (requires BACKEND_URLS config)
curl https://your-worker.dev/get

# Health check
curl https://your-worker.dev/_proxy/health
```

### Deployment
```bash
./deploy.sh  # Interactive deployment script
wrangler deploy --env production  # Direct deployment
```

## Configuration Examples

### Basic Load Balancer
```toml
[env.production.vars]
BACKEND_URLS = '["https://api1.example.com", "https://api2.example.com"]'
LOAD_BALANCER_STRATEGY = "round_robin"
HEALTH_CHECK_ENABLED = "true"
```

### Access Control
```toml
ACCESS_RULES = '[
  {"rule_type": "deny_country", "pattern": "CN"},
  {"rule_type": "allow_ip", "pattern": "192.168.1.0/24"}
]'
```

### Caching Setup
```toml
CACHE_ENABLED = "true"
CACHE_TTL = "300"

[[kv_namespaces]]
binding = "PROXY_KV"
id = "your-kv-namespace-id"
```

## Common Development Tasks

### Adding New Middleware
1. Extend `middleware.rs` functions: `apply_request_middleware()` or `apply_response_middleware()`
2. Update `ProxyConfig` in `config.rs` for new configuration options
3. Add environment variable parsing in `ProxyConfig::from_env()`

### New Load Balancing Strategy
1. Add enum variant to `LoadBalancerStrategy` in `load_balancer.rs`
2. Implement selection logic in `LoadBalancer::get_backend()`
3. Update `From<&str>` implementation for string parsing

### Adding Metrics
1. Extend `Metrics` struct in `monitoring.rs`
2. Add recording calls in `ReverseProxy::handle_request()`
3. Update `get_stats()` JSON response format

## Integration Points

### Cloudflare-Specific Features
- `req.cf()` provides country, IP, and edge location data
- `CF-Connecting-IP` header contains real client IP
- KV storage through `env.kv("PROXY_KV")` binding

### External Dependencies
- `worker` crate provides Cloudflare Workers runtime bindings
- `serde_json` for all configuration parsing (not `toml` - configs are JSON strings)
- `regex` for user-agent filtering and path rewriting
- `url` crate for URL parsing and manipulation in path proxy mode

## Debugging Common Issues

### Build Failures
- Ensure `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- Check `worker-build` version compatibility with `worker` crate

### Runtime Errors
- Check environment variable JSON syntax in wrangler.toml
- Verify KV namespace binding matches configuration
- Use `wrangler tail` for real-time error logging

### Performance Issues
- Monitor `/_proxy/stats` for cache hit rates and response times
- Check backend health status via `/_proxy/health`
- Review access control rules for unnecessary blocking
