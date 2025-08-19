format:
  taplo fmt
  cargo +nightly fmt --all
lint:
  taplo fmt --check
  cargo +nightly fmt --all -- --check
  cargo +nightly clippy --all -- -D warnings -A clippy::derive_partial_eq_without_eq -D clippy::unwrap_used -D clippy::uninlined_format_args
  cargo machete
test:
  cargo test
# Build project
build:
    npx wrangler build
# Start development server
dev:
    npx wrangler dev
# Deploy to production
deploy:
    npx wrangler deploy
# View real-time logs
logs:
    npx wrangler tail
# Clean build files
clean:
    rm -rf build/ target/