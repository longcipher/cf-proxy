# Configuration Examples

This document provides various configuration examples for different reverse proxy use cases.

## Basic Configuration Examples

### 1. Simple Reverse Proxy

```toml
# wrangler.toml
name = "simple-proxy"
main = "build/worker/shim.mjs"
compatibility_date = "2025-08-03"

[build]
command = "cargo install -q worker-build && worker-build --release"

[env.production.vars]
BACKEND_URLS = '["https://httpbin.org"]'
```

### 2. Multiple Backend Load Balancing

```json
{
  "BACKEND_URLS": [
    "https://api1.example.com",
    "https://api2.example.com", 
    "https://api3.example.com"
  ],
  "LOAD_BALANCER_STRATEGY": "round_robin",
  "HEALTH_CHECK_ENABLED": "true"
}
```

### 3. Enable Caching

```toml
[env.production.vars]
CACHE_ENABLED = "true"
CACHE_TTL = "300"

[[kv_namespaces]]
binding = "PROXY_KV"
id = "your-kv-namespace-id"
preview_id = "your-preview-kv-namespace-id"
```

## Advanced Configuration Examples

### 1. Complete Production Configuration

```toml
name = "production-cf-proxy"
main = "build/worker/shim.mjs"
compatibility_date = "2025-08-03"

[build]
command = "cargo install -q worker-build && worker-build --release"

[env.production.vars]
# Backend configuration
BACKEND_URLS = '[
  "https://api1.myservice.com",
  "https://api2.myservice.com",
  "https://api3.myservice.com"
]'

# Load balancing configuration
LOAD_BALANCER_STRATEGY = "round_robin"

# Health check configuration
HEALTH_CHECK_ENABLED = "true"
HEALTH_CHECK_INTERVAL = "30"
HEALTH_CHECK_TIMEOUT = "5"

# Cache configuration
CACHE_ENABLED = "true"
CACHE_TTL = "600"

# Timeout and retry configuration
TIMEOUT = "30"
RETRY_ATTEMPTS = "3"

# Log level
LOG_LEVEL = "info"

# Custom headers
CUSTOM_HEADERS = '{
  "X-Proxy-Version": "1.0",
  "X-Service-Name": "MyAPI",
  "X-Request-ID": "auto-generated"
}'

# Access control rules
ACCESS_RULES = '[
  {
    "rule_type": "deny_country",
    "pattern": "CN"
  },
  {
    "rule_type": "deny_ip",
    "pattern": "192.168.1.100"
  },
  {
    "rule_type": "deny_user_agent",
    "pattern": ".*bot.*"
  }
]'

# Path rewrite rules
PATH_REWRITE_RULES = '[
  {
    "pattern": "^/api/v1/(.*)",
    "replacement": "/v2/$1"
  },
  {
    "pattern": "^/legacy/(.*)",
    "replacement": "/modern/$1"
  }
]'

# KV namespace
[[kv_namespaces]]
binding = "PROXY_KV"
id = "your-production-kv-namespace-id"

# Staging environment configuration
[env.staging.vars]
BACKEND_URLS = '["https://staging-api.myservice.com"]'
CACHE_ENABLED = "false"
LOG_LEVEL = "debug"

[[env.staging.kv_namespaces]]
binding = "PROXY_KV"
id = "your-staging-kv-namespace-id"
```

### 2. Microservices Gateway Configuration

```toml
[env.production.vars]
# User service
USER_SERVICE_URLS = '["https://user1.api.com", "https://user2.api.com"]'

# Order service
ORDER_SERVICE_URLS = '["https://order1.api.com", "https://order2.api.com"]'

# Payment service
PAYMENT_SERVICE_URLS = '["https://payment1.api.com", "https://payment2.api.com"]'

# Path rewrite rules - route to different microservices
PATH_REWRITE_RULES = '[
  {
    "pattern": "^/api/users/(.*)",
    "replacement": "/users/$1",
    "target_service": "user"
  },
  {
    "pattern": "^/api/orders/(.*)",
    "replacement": "/orders/$1",
    "target_service": "order"
  },
  {
    "pattern": "^/api/payments/(.*)",
    "replacement": "/payments/$1",
    "target_service": "payment"
  }
]'
```

### 3. API Gateway Configuration

```toml
[env.production.vars]
# API backend
BACKEND_URLS = '["https://api.mycompany.com"]'

# API rate limiting configuration
RATE_LIMIT_ENABLED = "true"
RATE_LIMIT_REQUESTS_PER_MINUTE = "1000"

# API authentication configuration
AUTH_ENABLED = "true"
AUTH_HEADER = "Authorization"

# CORS configuration
CORS_ENABLED = "true"
CORS_ALLOWED_ORIGINS = '["https://myapp.com", "https://www.myapp.com"]'
CORS_ALLOWED_METHODS = '["GET", "POST", "PUT", "DELETE", "OPTIONS"]'
CORS_ALLOWED_HEADERS = '["Content-Type", "Authorization", "X-Requested-With"]'

# API version management
API_VERSION_HEADER = "X-API-Version"
DEFAULT_API_VERSION = "v2"

# Custom headers - API gateway identification
CUSTOM_HEADERS = '{
  "X-API-Gateway": "Cloudflare-Workers",
  "X-Gateway-Version": "1.0",
  "X-Rate-Limit-Remaining": "dynamic",
  "X-Response-Time": "auto"
}'
```

## Environment-Specific Configuration

### Development Environment

```toml
[env.development.vars]
BACKEND_URLS = '["http://localhost:3000"]'
CACHE_ENABLED = "false"
LOG_LEVEL = "debug"
HEALTH_CHECK_ENABLED = "false"
```

### Testing Environment

```toml
[env.testing.vars]
BACKEND_URLS = '["https://test-api.myservice.com"]'
CACHE_ENABLED = "true"
CACHE_TTL = "60"  # Shorter cache time for testing
LOG_LEVEL = "debug"
HEALTH_CHECK_INTERVAL = "10"  # More frequent health checks
```

### Production Environment

```toml
[env.production.vars]
BACKEND_URLS = '[
  "https://api1.myservice.com",
  "https://api2.myservice.com"
]'
CACHE_ENABLED = "true"
CACHE_TTL = "3600"  # 1 hour cache
LOG_LEVEL = "info"
HEALTH_CHECK_INTERVAL = "30"
RETRY_ATTEMPTS = "3"
TIMEOUT = "30"
```

## Security Configuration Examples

### 1. Geographic Access Control

```toml
[env.production.vars]
ACCESS_RULES = '[
  {
    "rule_type": "allow_country",
    "pattern": "US"
  },
  {
    "rule_type": "allow_country",
    "pattern": "CA"
  },
  {
    "rule_type": "deny_country",
    "pattern": "*"
  }
]'
```

### 2. IP Whitelist/Blacklist

```toml
[env.production.vars]
ACCESS_RULES = '[
  {
    "rule_type": "allow_ip",
    "pattern": "192.168.1.0/24"
  },
  {
    "rule_type": "allow_ip",
    "pattern": "10.0.0.0/8"
  },
  {
    "rule_type": "deny_ip",
    "pattern": "203.0.113.0/24"
  }
]'
```

### 3. User-Agent Filtering

```toml
[env.production.vars]
ACCESS_RULES = '[
  {
    "rule_type": "deny_user_agent",
    "pattern": ".*bot.*"
  },
  {
    "rule_type": "deny_user_agent",
    "pattern": ".*crawler.*"
  },
  {
    "rule_type": "deny_user_agent",
    "pattern": ".*spider.*"
  }
]'
```

## Cache Configuration Examples

### 1. Static Resource Caching

```toml
[env.production.vars]
CACHE_ENABLED = "true"
CACHE_TTL = "86400"  # 24 hours

# Cache rules
CACHE_RULES = '[
  {
    "pattern": "\\.(css|js|png|jpg|jpeg|gif|ico|svg)$",
    "ttl": 86400
  },
  {
    "pattern": "^/api/static/",
    "ttl": 3600
  }
]'
```

### 2. API Response Caching

```toml
[env.production.vars]
CACHE_ENABLED = "true"
CACHE_TTL = "300"  # 5 minutes default

# API-specific cache rules
CACHE_RULES = '[
  {
    "pattern": "^/api/users/profile$",
    "ttl": 1800,
    "vary": ["Authorization"]
  },
  {
    "pattern": "^/api/public/",
    "ttl": 3600,
    "public": true
  }
]'
```

## Monitoring Configuration Examples

### 1. Detailed Monitoring Configuration

```toml
[env.production.vars]
# Monitoring settings
MONITORING_ENABLED = "true"
METRICS_COLLECTION_INTERVAL = "60"
LOG_LEVEL = "info"

# Alert thresholds
ERROR_RATE_THRESHOLD = "5.0"  # 5%
RESPONSE_TIME_THRESHOLD = "2000"  # 2 seconds
UNHEALTHY_BACKEND_THRESHOLD = "50"  # 50%

# Monitoring endpoint configuration
HEALTH_CHECK_PATH = "/health"
METRICS_PATH = "/_proxy/stats"
```

### 2. Custom Metrics

```toml
[env.production.vars]
# Custom metrics collection
CUSTOM_METRICS = '[
  {
    "name": "api_calls_by_endpoint",
    "type": "counter",
    "labels": ["endpoint", "method", "status"]
  },
  {
    "name": "response_time_histogram",
    "type": "histogram",
    "buckets": [50, 100, 200, 500, 1000, 2000, 5000]
  }
]'
```

## Deployment Script Examples

### package.json

```json
{
  "name": "cloudflare-workers-cf-proxy",
  "version": "1.0.0",
  "scripts": {
    "dev": "wrangler dev",
    "deploy:staging": "wrangler deploy --env staging",
    "deploy:production": "wrangler deploy --env production",
    "logs": "wrangler tail",
    "test": "cargo test",
    "build": "cargo build --target wasm32-unknown-unknown --release"
  },
  "devDependencies": {
    "wrangler": "^3.0.0"
  }
}
```

### Deployment Script

```bash
#!/bin/bash
# deploy.sh

set -e

echo "Building Rust project..."
cargo build --target wasm32-unknown-unknown --release

echo "Running tests..."
cargo test

echo "Deploying to staging..."
wrangler deploy --env staging

echo "Running smoke tests..."
curl -f https://my-proxy-staging.workers.dev/_proxy/health

echo "Deploying to production..."
read -p "Deploy to production? (y/N) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    wrangler deploy --env production
    echo "Deployment complete!"
else
    echo "Production deployment cancelled."
fi
```

## Environment Variable Management

### Using wrangler secret

```bash
# Set sensitive configuration
wrangler secret put API_KEY
wrangler secret put DATABASE_URL
wrangler secret put JWT_SECRET

# List all secrets
wrangler secret list

# Delete secret
wrangler secret delete API_KEY
```

### Environment Variable Validation Script

```bash
#!/bin/bash
# validate-env.sh

required_vars=(
    "BACKEND_URLS"
    "LOAD_BALANCER_STRATEGY"
)

optional_vars=(
    "CACHE_ENABLED"
    "HEALTH_CHECK_ENABLED"
    "LOG_LEVEL"
)

echo "Validating environment configuration..."

for var in "${required_vars[@]}"; do
    if ! wrangler secret list | grep -q "$var"; then
        echo "❌ Missing required variable: $var"
        exit 1
    else
        echo "✅ Found required variable: $var"
    fi
done

echo "✅ All required variables are set"
```

This configuration examples file provides various configuration scenarios from basic to advanced, helping users choose the appropriate configuration for their needs.
