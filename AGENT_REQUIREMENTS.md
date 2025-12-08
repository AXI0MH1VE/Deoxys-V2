# ⚠️ CRITICAL AGENT REQUIREMENTS - READ FIRST

## 🚫 NO OS COMMANDS OR HOOKS - MANDATORY REQUIREMENT

**ALL AGENTS MUST READ AND COMPLY WITH THIS DOCUMENT**

### Core Principle: Zero External Dependencies

This codebase operates in a **completely isolated environment** with **ZERO OS command execution**. All functionality must run **in-process** using pure Rust implementations.

---

## ❌ FORBIDDEN OPERATIONS

### NEVER USE:
- ❌ `std::process::Command`
- ❌ `std::process::Command::new()`
- ❌ `std::process::spawn()`
- ❌ `std::process::exit()` (except for error handling in binaries)
- ❌ Any shell execution
- ❌ Any external process spawning
- ❌ Python subprocess calls
- ❌ System hooks or callbacks
- ❌ External script execution

### VERIFIED CLEAN MODULES:
✅ `src/core/toon-rs/src/lib.rs` - Pure parsing, zero network/OS operations  
✅ `src-tauri/src/main.rs` - All operations in-process  
✅ `src-tauri/src/mamba_core.rs` - Pure Rust Mamba-2 implementation  
✅ `src-tauri/src/fhe_core.rs` - Pure Rust FHE implementation  
✅ `src-tauri/src/contract_analyzer.rs` - Pure Rust contract analysis  
✅ `src-tauri/src/axiom_determinist/sandbox.rs` - Pure Rust validation (FIXED: removed OS commands)  

---

## ✅ REQUIRED IMPLEMENTATION PATTERN

### All Operations Must Be In-Process

**Example - CORRECT:**
```rust
// ✅ CORRECT: In-process implementation
use sha2::{Sha256, Digest};

pub fn encrypt_fhe(message: i32) -> Result<FHEResult, String> {
    // All computation happens in Rust, no OS calls
    let mut hasher = Sha256::new();
    hasher.update(&message.to_be_bytes());
    // ... pure Rust computation
    Ok(result)
}
```

**Example - WRONG:**
```rust
// ❌ WRONG: OS command execution
use std::process::Command;

pub fn encrypt_fhe(message: i32) -> Result<FHEResult, String> {
    let output = Command::new("python")  // ❌ FORBIDDEN
        .arg("script.py")
        .output()?;
    // ...
}
```

---

## 📋 MODULE VERIFICATION CHECKLIST

Before making any changes, verify:

1. **No `std::process` imports** - Search for `use std::process`
2. **No `Command::new` calls** - Search for `Command::`
3. **No external script execution** - All logic in Rust
4. **No network operations** - See `NETWORK_SAFETY.md`
5. **All dependencies are Rust crates** - No Python/Shell scripts

---

## 🔍 VERIFICATION COMMANDS

Run these checks before committing:

```bash
# Check for forbidden OS commands
grep -r "std::process::Command" src-tauri/src/
grep -r "Command::new" src-tauri/src/
grep -r "process::spawn" src-tauri/src/

# Should return NO results
```

---

## 📦 IMPLEMENTATION STATUS

### ✅ Completed In-Process Implementations

| Module | Status | Location |
|--------|--------|----------|
| TOON Parser | ✅ Pure Rust | `src/core/toon-rs/src/lib.rs` |
| Mamba-2 Model | ✅ Pure Rust | `src-tauri/src/mamba_core.rs` |
| FHE Encryption | ✅ Pure Rust | `src-tauri/src/fhe_core.rs` |
| Contract Analyzer | ✅ Pure Rust | `src-tauri/src/contract_analyzer.rs` |
| Risk Calculator | ✅ Pure Rust | `src/deployable/src/lib.rs` |
| AxiomDeterminist | ✅ Pure Rust | `src-tauri/src/axiom_determinist/` |
| - Sandbox Validation | ✅ Pure Rust | `src-tauri/src/axiom_determinist/sandbox.rs` |
| - Orchestrator | ✅ Pure Rust | `src-tauri/src/axiom_determinist/orchestrator.rs` |
| - DAG | ✅ Pure Rust | `src-tauri/src/axiom_determinist/dag.rs` |

### 🎯 Architecture Pattern

All modules follow this pattern:
1. **Pure Rust implementation** - No external processes
2. **Deterministic computation** - Zero Entropy Law (C=0)
3. **In-process execution** - All operations in same memory space
4. **No OS dependencies** - Self-contained within Rust runtime

---

## 🚨 ENFORCEMENT

### Code Review Requirements

**ALL pull requests must:**
1. Pass the verification commands above
2. Include a statement confirming no OS commands
3. Show that all functionality is in-process
4. Demonstrate pure Rust implementation

### Automatic Checks

The codebase includes:
- ✅ No `std::process::Command` imports
- ✅ All Python functionality ported to Rust
- ✅ All operations verified in-process
- ✅ Zero external script dependencies

---

## 📝 FOR AGENTS: What to Do

### When Adding New Features:

1. **Implement in Rust** - Never call external scripts
2. **Use existing modules** - Leverage `mamba_core.rs`, `fhe_core.rs`, etc.
3. **Verify no OS calls** - Run verification commands
4. **Document in-process** - Add comments showing pure Rust implementation

### When Modifying Existing Code:

1. **Check for OS commands** - Search for `Command::`, `spawn`, etc.
2. **Replace with Rust** - Port any external calls to Rust
3. **Test in-process** - Verify all operations are internal
4. **Update this doc** - If you add new modules, list them here

---

## 🔗 Related Documentation

- **Network Safety**: See `NETWORK_SAFETY.md` (if exists) or `src/core/toon-rs/src/lib.rs` comments
- **Zero Entropy Law**: See `ARCHITECTURE.md`
- **Code Style**: See `.cursorrules`

---

## ✅ VERIFICATION STATEMENT

**Current Status (as of last update):**
- ✅ Zero OS command execution in codebase
- ✅ All operations run in-process
- ✅ Pure Rust implementations for all modules
- ✅ No external script dependencies
- ✅ Complete isolation from OS environment

**Last Verified:** All modules checked and confirmed clean.

---

**⚠️ REMEMBER: If you see `std::process::Command` anywhere, it MUST be removed and replaced with pure Rust implementation.**

---

*This document must be read by all agents before making any code changes.*

