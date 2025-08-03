# Cloudflare Workers Rust Reverse Proxy

A feature-complete, configurable reverse proxy built with Rust, running on the Cloudflare Workers platform.

## Features

### 🚀 Core Features
- **Multi-backend Support** - Configure multiple backend servers
- **Load Balancing** - Round-robin, random, least connections strategies
- **Health Checks** - Automatic backend server health monitoring
- **Failover** - Automatic switching to healthy backend servers
- **URL Path Proxy** - Direct URL proxying via path (e.g., `https://your-worker.com/https://example.com/`)

### 🛡️ Security Features
- **Access Control** - IP, country, and User-Agent based access control
- **Security Headers** - Automatic security-related HTTP headers
- **Request Validation** - HMAC signature verification support
- **CORS Support** - Cross-Origin Resource Sharing headers for cross-domain requests

### ⚡ Performance Optimization
- **Smart Caching** - Cloudflare KV-based response caching
- **Path Rewriting** - Regular expression-based path rewrite rules
- **Header Processing** - Flexible request/response header modification
- **Redirect Handling** - Automatic redirect processing with proper path resolution

### 📊 Monitoring & Observability
- **Real-time Metrics** - Request counts, error rates, response time statistics
- **Health Check Endpoints** - Built-in health checks and status monitoring
- **Detailed Logging** - Structured logging

## Quick Start

### 1. Environment Setup

Ensure you have the following tools installed:

- [Rust](https://rustup.rs/) (latest stable version)
- [Node.js](https://nodejs.org/) and npm
- [wrangler CLI](https://developers.cloudflare.com/workers/wrangler/)

```bash
# Install Rust wasm32 target
rustup target add wasm32-unknown-unknown

# Install cargo-generate
cargo install cargo-generate

# Install wrangler globally
npm install -g wrangler
```

### 2. Usage Methods

This reverse proxy supports two main usage patterns:

#### Method 1: URL Path Proxy (Direct Proxying)

Simply make requests to your Cloudflare Workers URL with the target URL in the path:

```bash
# Proxy to example.com
https://your-worker-url.com/https://example.com/

# Proxy to specific path
https://your-worker-url.com/https://api.example.com/users/123

# Proxy with query parameters
https://your-worker-url.com/https://api.example.com/search?q=test&page=1
```

**Example requests:**
```bash
curl https://your-worker.workers.dev/https://httpbin.org/json
curl https://your-worker.workers.dev/https://api.github.com/users/octocat
```

#### Method 2: Load-Balanced Backend Proxy

Configure multiple backend servers for load balancing and health checks. This method requires configuration through environment variables.

### 3. Project Configuration

Edit the `wrangler.toml` file:

```toml
name = "my-cf-proxy"
main = "build/worker/shim.mjs"
compatibility_date = "2025-08-03"

[build]
command = "cargo install -q worker-build && worker-build --release"

# Environment variable configuration
[env.production.vars]
BACKEND_URLS = '["https://api1.example.com", "https://api2.example.com"]'
LOAD_BALANCER_STRATEGY = "round_robin"
HEALTH_CHECK_ENABLED = "true"
CACHE_ENABLED = "true"
CACHE_TTL = "300"

# KV namespace configuration
[[kv_namespaces]]
binding = "PROXY_KV"
id = "your-kv-namespace-id"
preview_id = "your-preview-kv-namespace-id"
```

### 3. Local Development

```bash
# Start local development server
wrangler dev

# Access in browser
# http://localhost:8787
```

### 4. Deploy to Production

```bash
# Deploy to Cloudflare Workers
wrangler deploy
```

## Key Features Detail

## URL Path Proxy

This feature allows you to use your Cloudflare Worker as a transparent proxy by embedding the target URL directly in the request path.

### Usage Pattern

Make requests to your Worker URL with the target URL appended to the path:

```
https://your-worker-url.com/https://example.com/path/to/resource
```

### Examples

1. **Basic usage:**

   ```http
   GET https://your-worker.example.workers.dev/https://api.github.com/users/octocat
   ```

2. **With query parameters:**

   ```http
   GET https://your-worker.example.workers.dev/https://httpbin.org/get?param1=value1&param2=value2
   ```

3. **POST requests with data:**

   ```http
   POST https://your-worker.example.workers.dev/https://api.example.com/data
   Content-Type: application/json
   
   {"key": "value"}
   ```

4. **Cross-origin requests from browser:**

   ```javascript
   fetch('https://your-worker.example.workers.dev/https://api.example.com/data', {
     method: 'POST',
     headers: {
       'Content-Type': 'application/json',
     },
     body: JSON.stringify({key: 'value'})
   })
   ```

### Proxy Features

- **Automatic redirect handling**: Follows HTTP redirects and proxies to the final destination
- **CORS support**: Adds proper CORS headers for cross-origin requests
- **Method preservation**: Maintains original HTTP methods (GET, POST, PUT, DELETE, etc.)
- **Headers forwarding**: Preserves original request headers (with necessary security filtering)
- **Binary content support**: Handles all content types including images, files, etc.
- **Query parameter preservation**: Maintains all query parameters from original request
- **OPTIONS preflight handling**: Automatically handles CORS preflight requests

### CORS Headers

The proxy automatically adds the following CORS headers:

- `Access-Control-Allow-Origin: *`
- `Access-Control-Allow-Methods: GET, POST, PUT, DELETE, PATCH, OPTIONS`
- `Access-Control-Allow-Headers: *`
- `Access-Control-Allow-Credentials: true`
- `Access-Control-Max-Age: 86400`

### How It Works

1. **URL Extraction**: The Worker extracts the target URL from the request path
2. **Request Forwarding**: Creates a new request to the target URL with original headers and body
3. **Redirect Handling**: If the response is a redirect (3xx), follows the redirect automatically
4. **Response Processing**: Returns the final response with appropriate CORS headers
5. **Error Handling**: Provides meaningful error messages for invalid URLs or failed requests

### Security Considerations

- The URL proxy feature is designed for development and testing purposes
- Be mindful of potential SSRF (Server-Side Request Forgery) vulnerabilities
- Consider implementing URL allowlists for production use
- Review and filter sensitive headers before forwarding requests
- Monitor usage to prevent abuse or excessive bandwidth consumption

### Cross-Origin Resource Sharing (CORS)

The reverse proxy automatically adds CORS headers to allow cross-domain requests:

```http
Access-Control-Allow-Origin: *
Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS, HEAD, PATCH
Access-Control-Allow-Headers: Content-Type, Authorization, X-Requested-With, Accept, Origin, User-Agent, DNT, Cache-Control, X-Mx-ReqToken, Keep-Alive, X-Requested-With, If-Modified-Since
Access-Control-Max-Age: 86400
Access-Control-Allow-Credentials: true
```

This enables frontend JavaScript applications to make requests across different domains without CORS restrictions.

### Redirect Processing

The proxy intelligently handles HTTP redirects (3xx status codes):

- **Relative Redirects**: Converts relative Location headers to absolute URLs
- **Absolute Redirects**: Preserves absolute redirect URLs
- **Path-based Redirects**: Resolves relative paths against the original target URL

## Configuration Options

### Environment Variables

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `BACKEND_URLS` | JSON Array | `["https://httpbin.org"]` | List of backend server URLs |
| `LOAD_BALANCER_STRATEGY` | String | `"round_robin"` | Load balancing strategy |
| `HEALTH_CHECK_ENABLED` | Boolean | `true` | Enable health checks |
| `HEALTH_CHECK_INTERVAL` | Number | `30` | Health check interval (seconds) |
| `CACHE_ENABLED` | Boolean | `false` | Enable caching |
| `CACHE_TTL` | Number | `300` | Cache TTL (seconds) |
| `CUSTOM_HEADERS` | JSON Object | `{}` | Custom request headers |
| `ACCESS_RULES` | JSON Array | `[]` | Access control rules |

### Load Balancing Strategies

- `round_robin` - Round Robin (default)
- `random` - Random selection
- `least_connections` - Least connections
- `weighted_round_robin` - Weighted round robin

### Access Control Rules Example

```json
[
  {
    "rule_type": "deny_ip",
    "pattern": "192.168.1.100"
  },
  {
    "rule_type": "allow_country",
    "pattern": "US"
  },
  {
    "rule_type": "deny_user_agent",
    "pattern": ".*bot.*"
  }
]
```

### Path Rewrite Rules Example

```json
[
  {
    "pattern": "^/api/v1/(.*)",
    "replacement": "/v2/$1"
  },
  {
    "pattern": "^/old-path/(.*)",
    "replacement": "/new-path/$1"
  }
]
```

## API Endpoints

### Management Endpoints

- `/_proxy/health` - Health check status
- `/_proxy/stats` - Proxy statistics

### Health Check Response Example

```json
{
  "status": "healthy",
  "healthy_backends": 2,
  "total_backends": 2,
  "backends": [
    "https://api1.example.com",
    "https://api2.example.com"
  ],
  "timestamp": "2025-08-03T12:00:00Z"
}
```

### Statistics Response Example

```json
{
  "total_requests": 1250,
  "total_errors": 15,
  "error_rate": "1.20%",
  "average_response_time": "125.50ms",
  "cache_hits": 450,
  "cache_misses": 800,
  "cache_hit_rate": "36.00%",
  "timestamp": "2025-08-03T12:00:00Z"
}
```

## Advanced Configuration

### Complete wrangler.toml Example

```toml
name = "advanced-cf-proxy"
main = "build/worker/shim.mjs"
compatibility_date = "2025-08-03"

[build]
command = "cargo install -q worker-build && worker-build --release"

[env.production.vars]
BACKEND_URLS = '["https://api1.example.com", "https://api2.example.com", "https://api3.example.com"]'
LOAD_BALANCER_STRATEGY = "round_robin"
HEALTH_CHECK_ENABLED = "true"
HEALTH_CHECK_INTERVAL = "30"
CACHE_ENABLED = "true"
CACHE_TTL = "600"
TIMEOUT = "30"
RETRY_ATTEMPTS = "3"

CUSTOM_HEADERS = '{
  "X-Custom-Header": "MyValue",
  "X-API-Version": "v2"
}'

ACCESS_RULES = '[
  {"rule_type": "deny_country", "pattern": "CN"},
  {"rule_type": "deny_user_agent", "pattern": ".*crawler.*"}
]'

PATH_REWRITE_RULES = '[
  {"pattern": "^/api/v1/(.*)", "replacement": "/v2/$1"},
  {"pattern": "^/legacy/(.*)", "replacement": "/modern/$1"}
]'

[[kv_namespaces]]
binding = "PROXY_KV"
id = "your-production-kv-id"

[env.staging.vars]
BACKEND_URLS = '["https://staging-api.example.com"]'
CACHE_ENABLED = "false"

[[env.staging.kv_namespaces]]
binding = "PROXY_KV"
id = "your-staging-kv-id"
```

## Monitoring and Debugging

### View Logs

```bash
# View Worker logs in real-time
wrangler tail
```

### Performance Monitoring

The proxy automatically records the following metrics:

- Total requests and error counts
- Average response time
- Cache hit rate
- Backend health status

### Debugging Tips

1. **Enable verbose logging**: Set `LOG_LEVEL=debug`
2. **Use health check endpoint**: Regularly check `/_proxy/health`
3. **Monitor statistics**: Get performance data through `/_proxy/stats`

## Best Practices

### Security Recommendations

1. **Use environment variables**: Manage sensitive configurations through environment variables or Workers Secrets
2. **Enable access control**: Configure appropriate IP and geographic restrictions
3. **Rotate keys regularly**: If using HMAC verification, update keys regularly

### Performance Optimization

1. **Configure caching appropriately**: Set suitable TTL based on content characteristics
2. **Health check intervals**: Balance detection timeliness and resource consumption
3. **Timeout settings**: Set appropriate request timeout values

### Operations Recommendations

1. **Monitoring alerts**: Set up monitoring alerts for key metrics
2. **Failure response plans**: Prepare emergency response plans for backend service failures
3. **Capacity planning**: Plan backend capacity based on traffic growth

## Troubleshooting

### Common Issues

1. **502 Bad Gateway**
   - Check if backend servers are running normally
   - Verify backend URL configuration is correct
   - Check health check status

2. **Request Timeout**
   - Adjust `TIMEOUT` configuration
   - Check network connectivity
   - Optimize backend service response time

3. **Cache Issues**
   - Verify KV namespace configuration
   - Check cache policy settings
   - Clear cache and retest

### Debug Commands

```bash
# Check configuration
wrangler whoami
wrangler kv:namespace list

# View KV storage
wrangler kv:key list --binding PROXY_KV

# Deploy before validation
wrangler deploy --dry-run
```

## Contributing

Welcome to submit Issues and Pull Requests!

### Development Workflow

1. Fork this repository
2. Create feature branch
3. Submit changes
4. Run tests
5. Submit Pull Request

### Code Standards

- Use `cargo fmt` to format code
- Use `cargo clippy` to check code quality
- Add appropriate test coverage

## License

This project uses the MIT license. See [LICENSE](LICENSE) file for details.

## Related Links

- [Cloudflare Workers Documentation](https://developers.cloudflare.com/workers/)
- [workers-rs Project](https://github.com/cloudflare/workers-rs)
- [Wrangler CLI Documentation](https://developers.cloudflare.com/workers/wrangler/)
