# 🚀 Quick Start Guide

This guide will help you depname = "my-cf-proxy"  # Change to your name
main = "build/worker/shim.mjs"
compatibility_date = "2025-08-03" and configure the Cloudflare Workers Rust reverse proxy in 5 minutes.

## 📋 Prerequisites

1. **Node.js and npm**

   ```bash
   node --version  # >= 16.x
   npm --version   # >= 8.x
   ```

2. **Rust and Cargo**

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup target add wasm32-unknown-unknown
   ```

3. **Cloudflare Account**
   - Sign up for a [Cloudflare account](https://dash.cloudflare.com/sign-up)
   - Get an [API Token](https://dash.cloudflare.com/profile/api-tokens)

## 🛠️ Install Tools

```bash
# Install Wrangler CLI
npm install -g wrangler

# Install worker-build
cargo install worker-build

# Login to Cloudflare
wrangler auth login
```

## ⚡ Quick Deployment

### 1. Clone and Build

```bash
cd cf-proxy
cargo check  # Verify code
worker-build --release  # Build WebAssembly
```

### 2. Configure wrangler.toml

Edit `wrangler.toml` and set your worker name:

```toml
name = "my-reverse-proxy"  # Change to your name
main = "build/worker/shim.mjs"
compatibility_date = "2023-12-01"
```

### 3. Set Environment Variables

```bash
# Basic configuration
wrangler secret put BACKEND_URLS
# Input: ["https://httpbin.org", "https://jsonplaceholder.typicode.com"]

wrangler secret put LOAD_BALANCER_STRATEGY
# Input: round_robin

# Optional: Advanced configuration
wrangler secret put HEALTH_CHECK_ENABLED
# Input: true

wrangler secret put CACHE_ENABLED  
# Input: true

wrangler secret put ACCESS_RULES
# Input: [{"rule_type": "allow_country", "pattern": "US"}]
```

### 4. Create KV Namespace (Optional)

If enabling cache functionality:

```bash
# Create KV namespace
wrangler kv:namespace create "PROXY_KV"

# Add to wrangler.toml
[[kv_namespaces]]
binding = "PROXY_KV"
id = "your-kv-namespace-id"
```

### 5. Deploy

```bash
./deploy.sh
# Or deploy manually
wrangler deploy
```

## 🧪 Test Deployment

After successful deployment, you can test the proxy functionality:

```bash
# Test basic functionality
curl https://my-cf-proxy.your-subdomain.workers.dev/get

# Check health status
curl https://my-cf-proxy.your-subdomain.workers.dev/_health

# View metrics
curl https://my-cf-proxy.your-subdomain.workers.dev/_metrics

# Check cache statistics
curl https://my-cf-proxy.your-subdomain.workers.dev/_cache/stats
```

## 📊 Monitoring and Debugging

```bash
# View real-time logs
wrangler tail

# Check deployment status
wrangler deployments list

# View KV data
wrangler kv:key list --namespace-id YOUR_KV_ID
```

## 🔧 Common Configurations

### Multi-backend Load Balancing
```json
{
  "BACKEND_URLS": ["https://api1.example.com", "https://api2.example.com"],
  "LOAD_BALANCER_STRATEGY": "least_connections",
  "HEALTH_CHECK_ENABLED": "true"
}
```

### Geographic Access Control

```json
{
  "ACCESS_RULES": [
    {"rule_type": "allow_country", "pattern": "US"},
    {"rule_type": "allow_country", "pattern": "CN"},
    {"rule_type": "deny_ip", "pattern": "192.168.1.100"}
  ]
}
```

### Cache Configuration

```json
{
  "CACHE_ENABLED": "true",
  "CACHE_TTL": "300",
  "CACHE_METHODS": "GET,HEAD"
}
```

## 🚨 Troubleshooting

### Common Issues

1. **Build Failed**

   ```bash
   # Clean cache and rebuild
   cargo clean
   worker-build --release
   ```

2. **Deployment Failed**

   ```bash
   # Check authentication status
   wrangler auth whoami
   
   # Check configuration
   wrangler kv:namespace list
   ```

3. **Runtime Errors**

   ```bash
   # View logs
   wrangler tail
   
   # Check environment variables
   wrangler secret list
   ```

### Performance Optimization

1. **Enable Caching**: Reduce requests to backends
2. **Health Checks**: Automatic failover
3. **Geographic Distribution**: Use multiple backend nodes
4. **Monitoring Alerts**: Set thresholds for key metrics

## 📈 Production Deployment Recommendations

1. **Custom Domain**: Configure in Cloudflare
2. **SSL/TLS**: Use Cloudflare SSL certificates
3. **Rate Limiting**: Configure Rate Limiting rules
4. **Backup**: Regular backup of configurations and data
5. **Monitoring**: Integrate Cloudflare Analytics

## 📚 More Resources

- [Complete Documentation](./README.md)
- [Configuration Examples](./examples.md)
- [Cloudflare Workers Documentation](https://developers.cloudflare.com/workers/)
- [Rust Workers Guide](https://developers.cloudflare.com/workers/languages/rust/)

---

🎉 **Congratulations!** Your Cloudflare Workers Rust reverse proxy has been successfully deployed!

If you encounter any issues, please check the logs or submit an Issue.
