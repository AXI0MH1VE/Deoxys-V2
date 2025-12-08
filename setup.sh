#!/bin/bash
# AxiomHive Sovereign Manifold v2.1.0 - Setup Script

echo "=== AxiomHive Sovereign Manifold v2.1.0 Setup ==="
echo "Zero Entropy Law (C=0) - Deterministic Intelligence System"
echo ""

# Check Python
if ! command -v python3 &> /dev/null; then
    echo "Error: Python 3 is required but not installed."
    exit 1
fi

# Check Rust
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust/Cargo is required but not installed."
    echo "Install from: https://rustup.rs/"
    exit 1
fi

# Check Node.js
if ! command -v node &> /dev/null; then
    echo "Error: Node.js is required but not installed."
    exit 1
fi

echo "Installing Python dependencies..."
pip3 install torch numpy

echo "Installing Node.js dependencies..."
npm install

echo "Building Rust components..."
cd src/core/toon-rs
cargo build --release --features frozen-seed
cd ../../..

cd src/deployable
cargo build --release --features frozen-seed
cd ../..

echo ""
echo "=== Setup Complete ==="
echo "Run 'npm run dev' to start the application"

