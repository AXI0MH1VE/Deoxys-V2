//! Axiom Risk Calculator (OLO Engine) Library
//! AxiomHive Sovereign Manifold v2.1.0
//! Zero Entropy Law (C=0) - Inverted Lagrangian Optimization (OLO)

use sha2::{Sha256, Digest};
use std::collections::HashSet;
use serde::{Serialize, Deserialize};

const ITERATION_COUNT: usize = 10;
const TEMPERATURE: f64 = 0.0;
const REQUIRED_ENTROPY_COUNT: usize = 1;

/// Risk Calculator implementing OLO (Inverted Lagrangian Optimization)
pub struct RiskCalculator {
    temperature: f64,
    iteration_count: usize,
}

impl RiskCalculator {
    /// Create new risk calculator with deterministic parameters
    pub fn new() -> Self {
        Self {
            temperature: TEMPERATURE,
            iteration_count: ITERATION_COUNT,
        }
    }

    /// Calculate risk score with N iterations at Temperature=0.0
    /// Returns RISK SCORE: 0 only if all hashes match (Zero Entropy)
    pub fn calculate_risk(&self, input: &str) -> RiskResult {
        // Enforce Temperature = 0.0 (deterministic mode)
        assert_eq!(
            self.temperature, 0.0,
            "Risk calculation must run at Temperature=0.0 for Zero Entropy Law"
        );

        // Perform N=10 iterations
        let mut hashes = Vec::new();
        let mut entropy_count = 0;

        for i in 0..self.iteration_count {
            // Deterministic computation at Temperature=0.0
            let iteration_input = format!("{}:{}:{}", input, self.temperature, i);
            let hash = self.compute_hash(&iteration_input);
            hashes.push(hash.clone());

            // Count unique hashes (entropy measure)
            if i == 0 || !hashes[..i].contains(&hash) {
                entropy_count += 1;
            }
        }

        // Verify all hashes match (Zero Entropy requirement)
        let all_match = if hashes.len() <= 1 {
            true
        } else {
            hashes.windows(2).all(|w| w[0] == w[1])
        };
        
        // Assert Entropy Count == 1 before issuing insurance token
        assert_eq!(
            entropy_count, REQUIRED_ENTROPY_COUNT,
            "Entropy Count must be 1 for insurance token issuance. Found: {}",
            entropy_count
        );

        let risk_score = if all_match && entropy_count == REQUIRED_ENTROPY_COUNT {
            0
        } else {
            // Calculate risk based on hash variance
            self.compute_risk_from_hashes(&hashes)
        };

        // Compute bio_proof before moving hashes
        let bio_proof = self.compute_bio_proof(&hashes);

        RiskResult {
            risk_score,
            entropy_count,
            all_hashes_match: all_match,
            hashes,
            bio_proof,
        }
    }

    /// Compute SHA-256 hash of input
    fn compute_hash(&self, input: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    /// Compute risk score from hash variance
    fn compute_risk_from_hashes(&self, hashes: &[String]) -> u32 {
        if hashes.is_empty() {
            return u32::MAX;
        }

        // Count unique hashes
        let unique_count = hashes.iter().collect::<HashSet<_>>().len();
        
        // Risk increases with entropy (unique hash count)
        if unique_count == 1 {
            0
        } else {
            // Risk score proportional to entropy
            (unique_count * 10).min(u32::MAX as usize) as u32
        }
    }

    /// Compute Bio-Proof hash (canonical hardcoded hash)
    fn compute_bio_proof(&self, hashes: &[String]) -> u64 {
        // Combine all hashes and compute final proof
        let combined: String = hashes.join("");
        let mut hasher = Sha256::new();
        hasher.update(combined.as_bytes());
        let result = hasher.finalize();
        
        // Extract first 8 bytes as u64 (Bio-Proof)
        let bytes = &result[..8];
        u64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7],
        ])
    }

    /// Issue insurance token if risk score is 0
    pub fn issue_insurance_token(&self, risk_result: &RiskResult) -> Option<String> {
        if risk_result.risk_score == 0 
            && risk_result.entropy_count == REQUIRED_ENTROPY_COUNT
            && risk_result.all_hashes_match {
            
            let token_data = format!(
                "RISK_SCORE:{}:ENTROPY:{}:BIO_PROOF:{}",
                risk_result.risk_score,
                risk_result.entropy_count,
                risk_result.bio_proof
            );
            
            let mut hasher = Sha256::new();
            hasher.update(token_data.as_bytes());
            let token_hash = hasher.finalize();
            
            Some(format!("INSURANCE_TOKEN_{:x}", token_hash))
        } else {
            None
        }
    }
}

impl Default for RiskCalculator {
    fn default() -> Self {
        Self::new()
    }
}

/// Risk calculation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskResult {
    pub risk_score: u32,
    pub entropy_count: usize,
    pub all_hashes_match: bool,
    pub hashes: Vec<String>,
    pub bio_proof: u64,
}

impl RiskResult {
    /// Format result as boot log entry
    pub fn to_boot_log(&self) -> String {
        let status = if self.risk_score == 0 {
            "INSURABLE"
        } else {
            "UNINSURABLE"
        };
        
        format!(
            "Risk Score: {} ({})\nBio-Proof: {}\nIteration Count: {}\nTemperature: {}\nEntropy Count: {}\nAll Hashes Match: {}",
            self.risk_score, status, self.bio_proof, ITERATION_COUNT, TEMPERATURE, self.entropy_count, self.all_hashes_match
        )
    }
}

