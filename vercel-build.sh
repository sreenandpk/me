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

echo "=== Downloading wasm-bindgen 0.2.127 Musl Static Binary ==="
WASM_BINDGEN_URL="https://github.com/rustwasm/wasm-bindgen/releases/download/0.2.127/wasm-bindgen-0.2.127-x86_64-unknown-linux-musl.tar.gz"
mkdir -p /tmp/wasm-bindgen
curl -sSL "$WASM_BINDGEN_URL" | tar -xz -C /tmp/wasm-bindgen --strip-components=1

echo "=== Compiling Rust WASM Application ==="
cargo build --target wasm32-unknown-unknown --release

echo "=== Packaging Web Application into dist/ ==="
mkdir -p dist/assets
cp index.html dist/
cp favicon.ico dist/ 2>/dev/null || true
cp favicon.png dist/ 2>/dev/null || true
if [ -d "assets" ]; then
    cp -r assets/* dist/assets/ 2>/dev/null || true
    cp -r assets/* dist/ 2>/dev/null || true
fi

# Run wasm-bindgen (exact version 0.2.127 matching Cargo.lock)
/tmp/wasm-bindgen/wasm-bindgen target/wasm32-unknown-unknown/release/portfolio.wasm --out-dir dist/assets --target web --no-typescript

# Inject WASM loader script into dist/index.html
sed -i 's|</body>|<script type="module">import init from "/assets/portfolio.js"; init();</script></body>|g' dist/index.html

echo "=== Web Application Build Completed Successfully ==="

