#!/usr/bin/env python3
"""
Mamba-2 Model Runner
Bridge script for Tauri to run Mamba-2 model
"""

#!/usr/bin/env python3
import sys
import os
import json
import argparse
import numpy as np

# Add parent directory to path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..'))
from src.core.mamba_core import create_mamba_core

def main():
    parser = argparse.ArgumentParser(description='Run Mamba-2 Model')
    parser.add_argument('--prompt', type=str, required=True)
    parser.add_argument('--state-dim', type=int, default=128)
    parser.add_argument('--input-dim', type=int, default=128)
    parser.add_argument('--temperature', type=float, default=0.0)
    parser.add_argument('--frozen-seed', type=str, default='true')
    
    args = parser.parse_args()
    
    try:
        # Create Mamba-2 core
        frozen_seed = args.frozen_seed.lower() == 'true'
        core = create_mamba_core(
            d_model=args.input_dim,
            d_state=args.state_dim,
            dt_rank=16,
            frozen_seed=frozen_seed
        )
        
        # Convert prompt to input vector (simplified - in production would use tokenizer)
        # For now, create a deterministic input from the prompt
        prompt_hash = hash(args.prompt) % (2**31)
        np.random.seed(prompt_hash if not frozen_seed else 42)
        input_vector = np.random.randn(args.input_dim).astype(np.float32)
        
        # Run model
        import torch
        input_tensor = torch.from_numpy(input_vector).unsqueeze(0).unsqueeze(0)  # [1, 1, d_model]
        
        with torch.no_grad():
            output = core(input_tensor)
        
        # Get stability metrics
        metrics = core.get_stability_metrics()
        
        # Format output
        output_text = f"Processed prompt: {args.prompt}\n"
        output_text += f"Output shape: {output.shape}\n"
        output_text += f"Temperature: {args.temperature}\n"
        output_text += f"State Dimension: {args.state_dim}\n"
        output_text += f"Frozen Seed: {frozen_seed}\n"
        
        # Calculate risk score (simplified)
        risk_score = 0 if args.temperature == 0.0 else 100
        
        result = {
            "output": output_text,
            "metrics": metrics,
            "risk_score": risk_score
        }
        
        print(json.dumps(result))
        
    except Exception as e:
        error_result = {
            "output": f"Error: {str(e)}",
            "metrics": None,
            "risk_score": 100
        }
        print(json.dumps(error_result))
        sys.exit(1)

if __name__ == "__main__":
    main()

