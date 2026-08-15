#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

echo "=== Verifying Rust Installation ==="
if ! command -v rustup &> /dev/null; then
    echo "rustup not found, installing..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo "rustup is already installed: $(rustup --version)"
fi

# Ensure cargo tools are in the PATH
export PATH="$HOME/.cargo/bin:$PATH"

echo "=== Adding WebAssembly Compilation Target ==="
rustup target add wasm32-unknown-unknown

echo "=== Installing cargo-binstall ==="
if ! command -v cargo-binstall &> /dev/null; then
    curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
else
    echo "cargo-binstall is already installed"
fi

echo "=== Installing Dioxus CLI (dx) ==="
if ! command -v dx &> /dev/null; then
    cargo binstall -y dioxus-cli
else
    echo "Dioxus CLI is already installed: $(dx --version)"
fi

echo "=== Compiling Dioxus Web Application (Static WASM) ==="
dx build --release

echo "=== Web Application Build Completed Successfully ==="
