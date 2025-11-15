#!/bin/bash
set -e

echo "🦀 Setting up Morpheus development environment..."

# Install WASM target
echo "📦 Installing wasm32-unknown-unknown target..."
rustup target add wasm32-unknown-unknown

# Install wasm-pack for WASM building
echo "📦 Installing wasm-pack..."
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install cargo-watch for auto-rebuild during development
echo "📦 Installing cargo-watch..."
cargo install cargo-watch

# Install wasm-bindgen-cli (matches the version in Cargo.toml)
echo "📦 Installing wasm-bindgen-cli..."
cargo install wasm-bindgen-cli

# Build the project to download all dependencies
echo "🔨 Building project and fetching dependencies..."
cargo build

echo "✅ Morpheus development environment ready!"
echo ""
echo "📚 Quick start:"
echo "  - Complete system: cd examples/morpheus-complete && cargo run --bin morpheus"
echo "  - Copy .env.example to .env and add your ANTHROPIC_API_KEY"
echo "  - Server will be available at http://localhost:3002"
echo ""
echo "🔍 Other examples:"
echo "  - Runtime compilation: cargo run --bin test-compiler"
echo "  - Full integration: cargo run --bin test-integration"
echo "  - Visual demo: cd examples/visual-demo && wasm-pack build --target web"
echo ""

npm install -g @anthropic-ai/claude-code
npm install -g @opencode