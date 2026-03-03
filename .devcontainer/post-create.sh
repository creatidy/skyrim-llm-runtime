#!/usr/bin/env bash
set -euo pipefail

echo "[devcontainer] Setting ownership of cached volumes..."
sudo chown -R vscode:vscode /home/vscode/.codex /home/vscode/.cargo

echo "[devcontainer] Bootstrapping Rust toolchain extras..."
rustup component add rustfmt clippy

echo "[devcontainer] Fetching workspace dependencies..."
cd runtime
cargo fetch

echo "[devcontainer] Running a quick compile check..."
cargo check

echo "[devcontainer] Ready."
