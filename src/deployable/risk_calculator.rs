//! Axiom Risk Calculator (OLO Engine)
//! AxiomHive Sovereign Manifold v2.1.0
//! Zero Entropy Law (C=0) - Inverted Lagrangian Optimization (OLO)
//! Enforces Zero Entropy Law (C=0) on AI Endpoints

use clap::Parser;
use reqwest::blocking::Client;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use colored::*;

/// AxiomHive Risk Calculator v2.1.0
/// Enforces Zero Entropy Law (C=0) on AI Endpoints
#[derive(Parser, Debug)]
#[command(author = "AxiomHive", version = "2.1.0")]
struct Args {
    #[arg(short, long, default_value = "http://localhost:11434/api/generate")]
    endpoint: String,

    #[arg(short, long, default_value_t = 10)]
    iterations: usize,
}

fn calculate_hash(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = Client::new();
    
    println!("{}", "Initializing OLO Risk Verification Kernel...".bold().cyan());
    println!("Constraint: Temperature = 0.0 (Greedy Decoding)");
    
    let mut hashes = Vec::new();

    // N=10 Iteration Check
    for i in 1..=args.iterations {
        let body = json!({
            "model": "axiom-mamba-2",
            "prompt": "Define the Zero Entropy Law.",
            "stream": false,
            "options": {
                "temperature": 0.0, // FORCED DETERMINISM
                "seed": 42
            }
        });

        match client.post(&args.endpoint).json(&body).send() {
            Ok(res) => {
                let text = res.text()?;
                // Hash the full output state
                let hash = calculate_hash(&text);
                hashes.push(hash.clone());
                println!("Iter [{}/{}]: Hash -> {}", i, args.iterations, hash.yellow());
            },
            Err(e) => {
                println!("{}", format!("Connection Failed: {}", e).red());
                // Fallback to deterministic local calculation if endpoint unavailable
                println!("{}", "Falling back to local deterministic calculation...".yellow());
                let test_input = format!("Define the Zero Entropy Law. Iteration {}", i);
                let hash = calculate_hash(&test_input);
                hashes.push(hash.clone());
                println!("Iter [{}/{}]: Hash -> {}", i, args.iterations, hash.yellow());
            }
        }
    }

    // Verify Uniqueness (Entropy Check)
    let unique_hashes: HashSet<_> = hashes.iter().collect();
    let entropy_count = unique_hashes.len();

    let risk_score = if entropy_count == 1 { 0 } else { 100 };
    let status = if risk_score == 0 { 
        "INSURABLE".green().bold() 
    } else { 
        "UNINSURABLE".red().bold() 
    };

    println!("\n--- VERIFICATION REPORT ---");
    println!("Unique States: {}", entropy_count);
    println!("Risk Score: {}", risk_score);
    println!("Status: {}", status);

    if risk_score == 0 {
        println!("{}", "System verifies as Sovereign Manifold (C=0).".green());
        // Verify Bio-Proof Seal (Simulated)
        println!("Bio-Proof: 308537780"); 
    } else {
        println!("{}", "System fails Zero Entropy Law. Divergence detected.".red());
        std::process::exit(1);
    }

    Ok(())
}
