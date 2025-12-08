#!/usr/bin/env python3
"""
Deoxys FHE Runner
Bridge script for Tauri to run FHE operations
"""

#!/usr/bin/env python3
import sys
import os
import json
import argparse
import base64

# Add parent directory to path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..'))
from src.security.fhe_wrapper import DeoxysFHE

def main():
    parser = argparse.ArgumentParser(description='Run Deoxys FHE Operations')
    parser.add_argument('--action', type=str, required=True, choices=['encrypt', 'decrypt'])
    parser.add_argument('--message', type=int, required=False)
    parser.add_argument('--ciphertext', type=str, required=False)
    parser.add_argument('--keys', type=str, required=False)
    
    args = parser.parse_args()
    
    try:
        fhe = DeoxysFHE(seed=b"AxiomHive_Frozen_Seed_v1.0")
        
        if args.action == 'encrypt':
            if args.message is None:
                raise ValueError("--message required for encryption")
            
            # Generate keys
            pk = fhe.keygen()
            
            # Encrypt
            ct = fhe.encrypt(pk, args.message)
            
            # Serialize for transport
            ct_str = json.dumps({
                'u': ct[0],
                'v': ct[1]
            })
            keys_str = json.dumps({
                'pk': pk,
                'sk': fhe.sk
            })
            
            result = {
                "ciphertext": base64.b64encode(ct_str.encode()).decode(),
                "keys": base64.b64encode(keys_str.encode()).decode()
            }
            
            print(json.dumps(result))
            
        elif args.action == 'decrypt':
            if args.ciphertext is None or args.keys is None:
                raise ValueError("--ciphertext and --keys required for decryption")
            
            # Deserialize
            keys_data = json.loads(base64.b64decode(args.keys).decode())
            ct_data = json.loads(base64.b64decode(args.ciphertext).decode())
            
            fhe.sk = keys_data['sk']
            ct = (ct_data['u'], ct_data['v'])
            
            # Decrypt
            plaintext = fhe.decrypt(fhe.sk, ct)
            
            print(str(plaintext))
            
    except Exception as e:
        print(f"Error: {str(e)}", file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    main()

