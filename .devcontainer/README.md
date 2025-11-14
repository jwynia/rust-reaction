# Morpheus Devcontainer Setup

This directory contains the VSCode devcontainer configuration for the Morpheus project, enabling consistent development environments using Docker containers.

## What's Included

### Base Environment
- **Rust toolchain** (stable, latest)
- **WASM support** (`wasm32-unknown-unknown` target)
- **Python 3.11** (for HTTP server in examples)
- **Node.js 20** (for web tooling)

### Rust Tools
- `wasm-pack` - Build and package WASM modules
- `cargo-watch` - Auto-rebuild on file changes
- `wasm-bindgen-cli` - WASM/JS interop bindings

### VSCode Extensions
- **rust-analyzer** - Rust LSP for intelligent code completion
- **vscode-lldb** - Debugging support
- **even-better-toml** - Cargo.toml syntax highlighting
- **crates** - Cargo dependency management
- **rust-syntax** - Enhanced Rust syntax highlighting

### Configuration
- **Auto-formatting** on save using rustfmt
- **Clippy** for linting on save
- **Port forwarding**:
  - 3002 (Morpheus main server)
  - 8080 (Examples HTTP server)
- **Cargo registry caching** for faster builds

## Getting Started

### Prerequisites
- [Docker Desktop](https://www.docker.com/products/docker-desktop)
- [VSCode](https://code.visualstudio.com/)
- [Dev Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)

### Opening in Devcontainer

1. **Open the project in VSCode**
   ```bash
   code /path/to/rust-reaction
   ```

2. **Reopen in container**
   - VSCode will prompt: "Folder contains a Dev Container configuration file. Reopen folder to develop in a container?"
   - Click **"Reopen in Container"**

   OR manually:
   - Press `F1` or `Ctrl+Shift+P`
   - Type "Dev Containers: Reopen in Container"
   - Press Enter

3. **Wait for setup**
   - First time: 5-10 minutes (downloads image, installs tools, builds project)
   - Subsequent times: ~30 seconds

4. **Set up environment**
   ```bash
   cd examples/morpheus-complete
   cp .env.example .env
   # Edit .env and add your ANTHROPIC_API_KEY
   ```

5. **Run the complete system**
   ```bash
   cargo run --bin morpheus
   ```
   - Server starts at http://localhost:3002
   - Ports are automatically forwarded to your host machine

## What Happens on First Start

The `post-create.sh` script automatically:
1. Installs WASM compilation target
2. Installs wasm-pack
3. Installs cargo-watch for development
4. Installs wasm-bindgen-cli
5. Builds the project and downloads all dependencies

## Common Tasks

### Run the complete Morpheus system
```bash
cd examples/morpheus-complete
cargo run --bin morpheus
# Visit http://localhost:3002
```

### Run individual phase examples
```bash
# Phase 1: Runtime compilation
cargo run --bin test-compiler

# Phase 3: Full integration
cargo run --bin test-integration
```

### Build WASM for visual demo
```bash
cd examples/visual-demo
wasm-pack build --target web
python -m http.server 8080
# Visit http://localhost:8080/public/
```

### Auto-rebuild on changes
```bash
cargo watch -x build
# Or for specific package:
cargo watch -p morpheus-core -x build
```

### Run tests
```bash
cargo test
# Or for specific package:
cargo test -p morpheus-core
```

## Performance Optimizations

### Cargo Registry Caching
The devcontainer mounts your host's cargo registry as a volume, so dependencies are shared across containers and persist between rebuilds.

### Incremental Builds
Cargo's incremental compilation is enabled by default, speeding up rebuilds.

## Troubleshooting

### Container won't start
- Check Docker Desktop is running
- Try: Dev Containers: Rebuild Container (rebuilds from scratch)

### Port already in use
- Check if something is running on ports 3002 or 8080 on your host
- Edit `.devcontainer/devcontainer.json` to use different ports

### Slow builds
- First build is always slow (downloads dependencies)
- Ensure cargo registry caching is working (check mounts in devcontainer.json)
- Consider increasing Docker Desktop memory allocation (Settings > Resources)

### Extensions not working
- Try: Developer: Reload Window (Ctrl+Shift+P)
- Check rust-analyzer output panel for errors

## Customization

### Add more tools
Edit `.devcontainer/post-create.sh` and add cargo install commands.

### Add VSCode extensions
Edit `.devcontainer/devcontainer.json` under `customizations.vscode.extensions`.

### Change Rust settings
Edit `.devcontainer/devcontainer.json` under `customizations.vscode.settings`.

## Differences from Local Development

✅ **Advantages:**
- Consistent environment across all developers
- No need to install Rust/WASM tooling locally
- Isolated from host system
- Easy onboarding for new contributors

⚠️ **Considerations:**
- Slightly slower file I/O (especially on macOS/Windows)
- Requires Docker Desktop
- First-time setup takes longer

## Environment Variables

The container sets:
- `RUST_BACKTRACE=1` - Full error backtraces

Add your own in `containerEnv` in devcontainer.json.

## Links

- [Devcontainer Documentation](https://code.visualstudio.com/docs/devcontainers/containers)
- [Rust Devcontainer Images](https://github.com/devcontainers/images/tree/main/src/rust)
- [Morpheus Project README](../README.md)
