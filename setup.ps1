# AxiomHive Sovereign Manifold v2.1.0 - Setup Script (PowerShell)

Write-Host "=== AxiomHive Sovereign Manifold v2.1.0 Setup ===" -ForegroundColor Cyan
Write-Host "Zero Entropy Law (C=0) - Deterministic Intelligence System" -ForegroundColor Gray
Write-Host ""

# Check Python
try {
    $pythonVersion = python --version 2>&1
    Write-Host "Found: $pythonVersion" -ForegroundColor Green
} catch {
    Write-Host "Error: Python is required but not installed." -ForegroundColor Red
    exit 1
}

# Check Rust
try {
    $rustVersion = cargo --version 2>&1
    Write-Host "Found: $rustVersion" -ForegroundColor Green
} catch {
    Write-Host "Error: Rust/Cargo is required but not installed." -ForegroundColor Red
    Write-Host "Install from: https://rustup.rs/" -ForegroundColor Yellow
    exit 1
}

# Check Node.js
try {
    $nodeVersion = node --version 2>&1
    Write-Host "Found: Node.js $nodeVersion" -ForegroundColor Green
} catch {
    Write-Host "Error: Node.js is required but not installed." -ForegroundColor Red
    exit 1
}

Write-Host "`nInstalling Python dependencies..." -ForegroundColor Yellow
pip install torch numpy

Write-Host "`nInstalling Node.js dependencies..." -ForegroundColor Yellow
npm install

Write-Host "`nBuilding Rust components..." -ForegroundColor Yellow
Set-Location src/core/toon-rs
cargo build --release --features frozen-seed
Set-Location ../../..

Set-Location src/deployable
cargo build --release --features frozen-seed
Set-Location ../..

Write-Host "`n=== Setup Complete ===" -ForegroundColor Green
Write-Host "Run 'npm run dev' to start the application" -ForegroundColor Cyan

