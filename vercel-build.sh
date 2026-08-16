#!/bin/bash
set -e

# Ensure PATH includes CARGO_HOME and HOME/.cargo
export CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"
export PATH="$CARGO_HOME/bin:$HOME/.cargo/bin:$PATH"

echo "=== Verifying Rust Installation ==="
if ! command -v rustup &> /dev/null; then
    echo "rustup not found, installing..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$CARGO_HOME/env" 2>/dev/null || source "$HOME/.cargo/env" 2>/dev/null || true
else
    echo "rustup is already installed: $(rustup --version)"
fi

echo "=== Adding WebAssembly Compilation Target ==="
rustup target add wasm32-unknown-unknown

echo "=== Installing Dioxus CLI / WASM Toolchain ==="
if ! command -v dx &> /dev/null; then
    echo "Downloading prebuilt dx binary..."
    mkdir -p "$CARGO_HOME/bin"
    
    # Download static musl release binary from GitHub Releases
    URL="https://github.com/DioxusLabs/dioxus/releases/download/v0.6.2/dx-v0.6.2-x86_64-unknown-linux-musl.tar.gz"
    if curl -sSLf "$URL" -o /tmp/dx.tar.gz 2>/dev/null; then
        tar -xzf /tmp/dx.tar.gz -C "$CARGO_HOME/bin"
        chmod +x "$CARGO_HOME/bin/dx" 2>/dev/null || true
    fi
fi

if command -v dx &> /dev/null; then
    echo "Using Dioxus CLI: $(dx --version)"
    echo "=== Compiling Dioxus Web Application (dx build) ==="
    dx build --release
else
    echo "=== Compiling Dioxus Web Application (cargo WASM fallback) ==="
    WASM_BINDGEN_URL="https://github.com/rustwasm/wasm-bindgen/releases/download/0.2.100/wasm-bindgen-0.2.100-x86_64-unknown-linux-musl.tar.gz"
    mkdir -p /tmp/wasm-bindgen
    curl -sSL "$WASM_BINDGEN_URL" | tar -xz -C /tmp/wasm-bindgen --strip-components=1
    
    cargo build --target wasm32-unknown-unknown --release
    
    mkdir -p dist/assets
    cp index.html dist/
    if [ -d "assets" ]; then
        cp -r assets/* dist/assets/ 2>/dev/null || true
    fi
    
    /tmp/wasm-bindgen/wasm-bindgen target/wasm32-unknown-unknown/release/portfolio.wasm --out-dir dist/assets --target web --no-typescript
fi

echo "=== Web Application Build Completed Successfully ==="

