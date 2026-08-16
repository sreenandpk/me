#!/bin/bash
set -e

# Ensure PATH includes both CARGO_HOME and HOME/.cargo
export CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"
export PATH="$CARGO_HOME/bin:$HOME/.cargo/bin:$PATH"

echo "=== Verifying & Updating Rust Installation ==="
if ! command -v rustup &> /dev/null; then
    echo "rustup not found, installing..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$CARGO_HOME/env" 2>/dev/null || source "$HOME/.cargo/env" 2>/dev/null || true
else
    echo "rustup is already installed: $(rustup --version)"
fi

# Update rust compiler to latest stable so dependencies compile cleanly
rustup update stable
rustup default stable

echo "=== Adding WebAssembly Compilation Target ==="
rustup target add wasm32-unknown-unknown

echo "=== Installing Dioxus CLI (dx) ==="
if ! command -v dx &> /dev/null; then
    echo "Installing Dioxus CLI natively on build container..."
    cargo install dioxus-cli
else
    echo "Dioxus CLI is already installed: $(dx --version)"
fi

echo "=== Compiling Dioxus Web Application (Static WASM) ==="
dx build --release

echo "=== Web Application Build Completed Successfully ==="

