#!/usr/bin/env python3
"""
Contract Pipeline Runner
Bridge script for Tauri to run contract summarization
"""

import sys
import os
import json
import argparse

# Add parent directory to path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..'))
from src.core.contract_pipeline import ContractPipeline

def main():
    parser = argparse.ArgumentParser(description='Run Contract Pipeline')
    parser.add_argument('--contract', type=str, required=True)
    parser.add_argument('--frozen-seed', type=str, default='PLACEHOLDER_UUID')
    
    args = parser.parse_args()
    
    try:
        pipeline = ContractPipeline(frozen_seed=args.frozen_seed)
        result = pipeline.process_contract(args.contract)
        
        print(json.dumps(result))
        
    except Exception as e:
        error_result = {
            "status": "error",
            "error": {
                "message": str(e),
                "error_codes": ["PROCESSING_ERROR"]
            }
        }
        print(json.dumps(error_result))
        sys.exit(1)

if __name__ == "__main__":
    main()

