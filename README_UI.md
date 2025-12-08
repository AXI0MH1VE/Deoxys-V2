# AxiomHive UI Setup and Usage Guide

## Quick Start

### Prerequisites
- Rust (latest stable)
- Python 3.11+ with PyTorch installed
- Node.js 18+ (for Tauri)
- npm or yarn

### Installation

1. **Install Python dependencies:**
```bash
pip install torch numpy
```

2. **Install Node.js dependencies:**
```bash
npm install
```

3. **Build and run:**
```bash
npm run dev
```

## UI Features

### 1. Mamba-2 Model Control
- **Input Prompt**: Enter text prompts for the model
- **State Dimension**: Control the hidden state size (64-256)
- **Input Dimension**: Control input vector size (64-512)
- **Temperature**: Set temperature for generation (0.0 = deterministic)
- **Frozen Seed**: Enable/disable deterministic seed

### 2. TOON Parser
- Parse Token-Oriented Object Notation data
- Format: `key [count]{schema}`
- Example: `market_ticks [1000]{symbol,price,vol,ts}`

### 3. Risk Verification
- Verify system determinism
- Check entropy count
- Calculate risk score (0 = INSURABLE)

### 4. Deoxys FHE
- Encrypt numbers using Fully Homomorphic Encryption
- Decrypt previously encrypted data
- LWE lattice-based security

## Architecture

- **Frontend**: HTML/CSS/JavaScript with Tailwind CSS
- **Backend**: Rust (Tauri) with Python bridges
- **Model**: PyTorch-based Mamba-2 State Space Model
- **Security**: Deoxys FHE with LWE parameters

## File Structure

```
Deoxys/
├── index.html              # Main UI
├── src/
│   └── main.js            # Frontend controller
├── src-tauri/
│   ├── src/
│   │   └── main.rs        # Tauri backend
│   └── Cargo.toml         # Rust dependencies
├── src/core/
│   ├── mamba_core.py      # Mamba-2 model
│   └── mamba_runner.py   # Python bridge
└── src/security/
    ├── fhe_wrapper.py     # FHE implementation
    └── fhe_runner.py      # Python bridge
```

## Usage Examples

### Running the Model
1. Enter a prompt in the "Input Prompt" field
2. Adjust parameters (State Dim, Input Dim, Temperature)
3. Click "Run Model"
4. View output in the "Model Output" section

### Parsing TOON Data
1. Enter TOON format data: `tokens [100]{string}`
2. Click "Parse TOON"
3. View parsed structure

### Encrypting with FHE
1. Enter a number (0-65535)
2. Click "Encrypt"
3. Copy the ciphertext and keys
4. Use "Decrypt" to recover the original number

## Troubleshooting

### Python not found
- Ensure Python is in your PATH
- Use `python3` instead of `python` if needed
- Update the command in `src-tauri/src/main.rs`

### PyTorch not installed
```bash
pip install torch
```

### Tauri build errors
```bash
npm install @tauri-apps/cli --save-dev
```

## Zero Entropy Law Compliance

All operations enforce C=0:
- Temperature defaults to 0.0 (deterministic)
- Frozen seed enabled by default
- Risk score must be 0 for INSURABLE status
- Entropy count must be 1

