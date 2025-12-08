# AxiomHive Sovereign Manifold v2.1.0

**Zero Entropy Law (C=0) - Deterministic Intelligence System**

A fully functional desktop application that replaces probabilistic "Generative AI" with Higher-Dimensional Deterministic Intelligence, enforcing complete determinism across all subsystems.

## 🚀 Quick Start

### Prerequisites
- **Rust** (latest stable) - [Install Rust](https://rustup.rs/)
- **Python 3.11+** with pip
- **Node.js 18+** and npm
- **PyTorch** (will be installed automatically)

### Installation

**Windows:**
```powershell
.\setup.ps1
```

**Linux/macOS:**
```bash
chmod +x setup.sh
./setup.sh
```

**Manual Installation:**
```bash
# Install Python dependencies
pip install torch numpy

# Install Node.js dependencies
npm install

# Build Rust components
cd src/core/toon-rs && cargo build --release --features frozen-seed && cd ../../..
cd src/deployable && cargo build --release --features frozen-seed && cd ../..
```

### Running the Application

```bash
npm run dev
```

This will:
1. Start the Tauri development server
2. Open the AxiomHive desktop application
3. Allow you to interact with all system components

## 📦 What's Included

### Core Components

1. **TOON Parser** - Zero-copy data serialization format
2. **Mamba-2 Model** - Linear complexity State Space Model with deterministic HiPPO initialization
3. **Deoxys FHE** - Fully Homomorphic Encryption with LWE lattice parameters
4. **OLO Risk Engine** - Inverted Lagrangian Optimization with SHA-256 verification
5. **Visual Theme OS** - Canonical UI with Axiom Black (#000000) and Miami Red (#FF0038)

### User Interface Features

- **Model Control Panel**: Run Mamba-2 with adjustable parameters
  - State Dimension (64-256)
  - Input Dimension (64-512)
  - Temperature (0.0 = deterministic)
  - Frozen Seed toggle
  
- **TOON Parser**: Parse Token-Oriented Object Notation data
- **Risk Verification**: Verify system determinism and get risk scores
- **FHE Encryption**: Encrypt/decrypt numbers using Fully Homomorphic Encryption
- **System Status**: Real-time monitoring of all components

## 🏗️ Architecture

```
Deoxys/
├── index.html              # Main UI
├── src/
│   ├── main.js           # Frontend controller
│   ├── core/
│   │   ├── toon-rs/      # TOON parser (Rust)
│   │   ├── mamba_core.py # Mamba-2 model
│   │   └── mamba_runner.py # Python bridge
│   ├── security/
│   │   ├── fhe_wrapper.py # Deoxys FHE
│   │   └── fhe_runner.py  # Python bridge
│   └── deployable/
│       └── risk_calculator.rs # OLO Risk Engine
├── src-tauri/
│   ├── src/main.rs       # Tauri backend
│   └── Cargo.toml        # Rust dependencies
└── ui/
    ├── global.css        # Visual Theme OS
    └── tailwind.config.js
```

## 🔧 Building for Production

```bash
npm run build
```

This creates platform-specific installers in `src-tauri/target/release/`.

## ✅ Verification

The system enforces the Zero Entropy Law (C=0):

- ✅ TOON parser rejects JSON delimiters
- ✅ Mamba-2 uses deterministic HiPPO initialization
- ✅ Risk calculator enforces Temperature=0.0 and Entropy Count == 1
- ✅ Visual Theme OS enforces canonical palette
- ✅ All components use frozen-seed feature

### Expected Boot Log

```
Risk Score: 0 (INSURABLE)
Bio-Proof: 308537780
Iteration Count: 10
Temperature: 0.0
Entropy Count: 1
All Hashes Match: true
```

## 📖 Usage Examples

### Running the Mamba-2 Model

1. Open the application
2. Enter a prompt in the "Input Prompt" field
3. Adjust parameters (State Dim, Input Dim, Temperature)
4. Click "Run Model"
5. View output and stability metrics

### Parsing TOON Data

Enter TOON format: `market_ticks [1000]{symbol,price,vol,ts}` and click "Parse TOON"

### Encrypting with FHE

1. Enter a number (0-65535)
2. Click "Encrypt"
3. Copy the ciphertext
4. Use "Decrypt" to recover the original

## 🐛 Troubleshooting

### Python not found
- Ensure Python is in your PATH
- Use `python3` instead of `python` if needed
- Update paths in `src-tauri/src/main.rs`

### PyTorch installation fails
```bash
pip install torch --index-url https://download.pytorch.org/whl/cpu
```

### Tauri build errors
```bash
npm install @tauri-apps/cli --save-dev
```

### Module import errors
Ensure you're running from the project root directory.

## 📄 License

MIT License - AxiomHive Sovereign Manifold

## 🔗 Links

- **GitHub**: https://github.com/AXI0MH1VE/Deoxys-V2
- **Architecture Docs**: See `ARCHITECTURE.md`
- **UI Guide**: See `README_UI.md`

---

**AxiomHive Sovereign Manifold v2.1.0** | Zero Entropy Law (C=0) | Bio-Proof: 308537780
