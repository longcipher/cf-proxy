#!/bin/bash

# Cloudflare Workers Rust Reverse Proxy Deployment Script
set -e

echo "🚀 Deploying Cloudflare Workers Rust Reverse Proxy..."

# Check necessary commands
if ! command -v wrangler &> /dev/null; then
    echo "❌ Error: wrangler CLI not installed"
    echo "Please run: npm install -g wrangler"
    exit 1
fi

if ! command -v worker-build &> /dev/null; then
    echo "❌ Error: worker-build not installed"
    echo "Please run: cargo install worker-build"
    exit 1
fi

# Build project
echo "🔧 Building WebAssembly..."
worker-build --release

# Check wrangler.toml configuration
if [ ! -f "wrangler.toml" ]; then
    echo "❌ Error: wrangler.toml configuration file does not exist"
    exit 1
fi

# Prompt user for configuration
echo "📝 Please ensure you have configured:"
echo "   1. name field in wrangler.toml"
echo "   2. Environment variables (via wrangler secret put)"
echo "   3. KV namespace (if using cache functionality)"
echo ""

# Ask if continue
read -p "Continue with deployment? (y/N): " confirm
if [[ $confirm != [yY] && $confirm != [yY][eE][sS] ]]; then
    echo "❌ Deployment cancelled"
    exit 0
fi

# Deploy to Cloudflare Workers
echo "☁️  Deploying to Cloudflare Workers..."
wrangler deploy

echo "✅ Deployment complete!"
echo ""
echo "📋 Next steps:"
echo "   1. Configure environment variables in Cloudflare dashboard"
echo "   2. Set up custom domain (optional)"
echo "   3. Configure access rules and load balancing"
echo "   4. Test proxy functionality"
echo ""
echo "📊 Monitoring commands:"
echo "   wrangler tail          # View real-time logs"
echo "   wrangler kv:namespace  # Manage KV storage"
echo "   wrangler secret        # Manage environment variables"
