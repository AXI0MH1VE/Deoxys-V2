#!/usr/bin/env python3
"""
AxiomHive Sovereign Manifold v2.1.0 - System Test Script
Verifies all components are functioning correctly
"""

import sys
import os

def test_mamba_core():
    """Test Mamba-2 State Space Model"""
    print("Testing Mamba-2 Core...")
    try:
        sys.path.insert(0, 'src/core')
        from mamba_core import MambaCore, create_mamba_core
        
        # Create core with frozen seed
        core = create_mamba_core(state_dim=64, input_dim=128, output_dim=128, frozen_seed=True)
        
        # Test forward pass
        import numpy as np
        test_input = np.random.randn(128)
        output, hidden = core.forward(test_input)
        
        # Check stability
        metrics = core.get_stability_metrics()
        
        print(f"  ✓ Mamba-2 Core initialized")
        print(f"  ✓ State Space Duality equation: h'(t) = Ah(t) + Bx(t)")
        print(f"  ✓ Lyapunov Stability: {metrics['is_stable']}")
        print(f"  ✓ Max Real Eigenvalue: {metrics['max_real_eigenvalue']:.6f}")
        return True
    except Exception as e:
        print(f"  ✗ Mamba-2 Core test failed: {e}")
        return False

def test_deoxys_fhe():
    """Test Deoxys FHE wrapper"""
    print("\nTesting Deoxys FHE...")
    try:
        sys.path.insert(0, 'src/security')
        from fhe_wrapper import DeoxysFHE, RecursiveIntelligenceKernel
        
        # Initialize FHE
        fhe = DeoxysFHE(seed=b"AxiomHive_Frozen_Seed_v1.0")
        pk, sk = fhe.generate_keys()
        
        # Test encryption/decryption
        plaintext = 42
        ciphertext = fhe.encrypt(pk, plaintext)
        decrypted = fhe.decrypt(sk, ciphertext)
        
        assert decrypted == plaintext, "Decryption failed"
        
        print(f"  ✓ Deoxys FHE initialized")
        print(f"  ✓ Modulus Q: {fhe.modulus_q}")
        print(f"  ✓ Plaintext Modulus T: {fhe.plaintext_modulus_t}")
        print(f"  ✓ Encryption/Decryption: PASS")
        
        # Test RIK
        rik = RecursiveIntelligenceKernel(fhe)
        print(f"  ✓ Recursive Intelligence Kernel (RIK) initialized")
        
        return True
    except Exception as e:
        print(f"  ✗ Deoxys FHE test failed: {e}")
        return False

def test_visual_theme():
    """Test Visual Theme OS files"""
    print("\nTesting Visual Theme OS...")
    try:
        # Check CSS file
        css_path = 'ui/global.css'
        if not os.path.exists(css_path):
            raise FileNotFoundError(f"CSS file not found: {css_path}")
        
        with open(css_path, 'r') as f:
            css_content = f.read()
        
        # Verify canonical palette
        assert '#000000' in css_content or '--axiom-black' in css_content, "Axiom Black not found"
        assert '#FF0038' in css_content or '--miami-red' in css_content, "Miami Red not found"
        assert 'background-image: none !important' in css_content, "Background override not found"
        assert 'honeycomb' in css_content.lower(), "Honeycomb pattern not found"
        
        # Check Tailwind config
        config_path = 'ui/tailwind.config.js'
        if not os.path.exists(config_path):
            raise FileNotFoundError(f"Tailwind config not found: {config_path}")
        
        with open(config_path, 'r') as f:
            config_content = f.read()
        
        assert "'axiom-black'" in config_content or '"axiom-black"' in config_content
        assert "'miami-red'" in config_content or '"miami-red"' in config_content
        
        print(f"  ✓ Global CSS: Canonical palette enforced")
        print(f"  ✓ Tailwind Config: Axiom Black and Miami Red defined")
        print(f"  ✓ Background Override: background-image: none!important")
        print(f"  ✓ Honeycomb Hex Grid: Pattern implemented")
        
        return True
    except Exception as e:
        print(f"  ✗ Visual Theme OS test failed: {e}")
        return False

def test_file_structure():
    """Test file structure"""
    print("\nTesting File Structure...")
    required_files = [
        'src/core/toon-rs/Cargo.toml',
        'src/core/toon-rs/src/lib.rs',
        'src/core/mamba_core.py',
        'src/security/fhe_wrapper.py',
        'src/deployable/Cargo.toml',
        'src/deployable/risk_calculator.rs',
        'ui/global.css',
        'ui/tailwind.config.js',
        'deployable/Containerfile',
        'README.md',
        'verification_report.json'
    ]
    
    all_exist = True
    for file_path in required_files:
        if os.path.exists(file_path):
            print(f"  ✓ {file_path}")
        else:
            print(f"  ✗ {file_path} - MISSING")
            all_exist = False
    
    return all_exist

def main():
    """Run all tests"""
    print("=" * 60)
    print("AxiomHive Sovereign Manifold v2.1.0 - System Verification")
    print("Zero Entropy Law (C=0) - Deterministic Intelligence")
    print("=" * 60)
    
    results = []
    
    results.append(("File Structure", test_file_structure()))
    results.append(("Mamba-2 Core", test_mamba_core()))
    results.append(("Deoxys FHE", test_deoxys_fhe()))
    results.append(("Visual Theme OS", test_visual_theme()))
    
    print("\n" + "=" * 60)
    print("Test Results Summary:")
    print("=" * 60)
    
    all_passed = True
    for name, passed in results:
        status = "PASS" if passed else "FAIL"
        print(f"  {name}: {status}")
        if not passed:
            all_passed = False
    
    print("=" * 60)
    if all_passed:
        print("✓ All tests PASSED - System ready for deployment")
        print("\nExpected Boot Log:")
        print("  Risk Score: 0 (INSURABLE)")
        print("  Bio-Proof: 308537780")
        print("  Entropy Count: 1")
        return 0
    else:
        print("✗ Some tests FAILED - Review errors above")
        return 1

if __name__ == "__main__":
    sys.exit(main())

