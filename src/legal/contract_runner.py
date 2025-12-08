#!/usr/bin/env python3
"""
Contract Analyzer Runner
Bridge script for Tauri to run contract analysis
"""

import sys
import os
import json
import argparse

# Add parent directory to path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..'))
from src.legal.contract_analyzer import ContractAnalyzer

def main():
    parser = argparse.ArgumentParser(description='Analyze Contract')
    parser.add_argument('--contract-text', type=str, required=True)
    parser.add_argument('--frozen-seed', type=str, default='true')
    
    args = parser.parse_args()
    
    try:
        frozen_seed = args.frozen_seed.lower() == 'true'
        analyzer = ContractAnalyzer(frozen_seed=frozen_seed)
        
        result = analyzer.analyze_contract(args.contract_text)
        
        print(json.dumps(result))
        
    except Exception as e:
        error_result = {
            'status': 'error',
            'error': str(e),
            'failure_codes': ['SCHEMA_VIOLATION']
        }
        print(json.dumps(error_result))
        sys.exit(1)

if __name__ == "__main__":
    main()

