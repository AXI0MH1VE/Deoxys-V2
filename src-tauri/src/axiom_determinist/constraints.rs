// Tier 2: Constraint-Based Generation
// Logit bias, token banning, and grammar constraints

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Logit bias configuration for token banning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogitBias {
    /// Map of token ID to bias value (-100 effectively bans the token)
    pub token_biases: HashMap<u32, f32>,
    /// Banned token strings (will be converted to token IDs)
    pub banned_strings: Vec<String>,
}

impl LogitBias {
    pub fn new() -> Self {
        Self {
            token_biases: HashMap::new(),
            banned_strings: vec![
                "TODO".to_string(),
                "FIXME".to_string(),
                "XXX".to_string(),
                "HACK".to_string(),
                "todo".to_string(),
                "fixme".to_string(),
                "xxx".to_string(),
                "hack".to_string(),
                "NotImplementedError".to_string(),
                "NotImplemented".to_string(),
                "pass".to_string(),
                "return null".to_string(),
                "return None".to_string(),
                "omitted for brevity".to_string(),
                "rest of code".to_string(),
                "left as an exercise".to_string(),
                "implementation omitted".to_string(),
            ],
        }
    }

    /// Convert banned strings to token IDs using tokenizer
    /// This would integrate with tiktoken (OpenAI) or the model's tokenizer
    pub fn apply_tokenizer(&mut self, tokenizer: &dyn Tokenizer) {
        for banned_str in &self.banned_strings {
            let token_ids = tokenizer.encode(banned_str);
            for token_id in token_ids {
                // Set bias to -100 to effectively ban the token
                self.token_biases.insert(token_id, -100.0);
            }
        }
    }

    /// Get logit bias map for API call
    pub fn get_bias_map(&self) -> &HashMap<u32, f32> {
        &self.token_biases
    }
}

impl Default for LogitBias {
    fn default() -> Self {
        Self::new()
    }
}

/// Grammar constraint for syntax enforcement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrammarConstraint {
    pub language: ProgrammingLanguage,
    pub grammar_rules: Vec<GrammarRule>,
    pub forbidden_constructs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProgrammingLanguage {
    Python,
    Rust,
    JavaScript,
    TypeScript,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrammarRule {
    pub rule_name: String,
    pub ebnf_definition: String,
    pub enforcement: EnforcementLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnforcementLevel {
    Warning,
    Error,
    Fatal,
}

impl GrammarConstraint {
    pub fn for_python() -> Self {
        Self {
            language: ProgrammingLanguage::Python,
            grammar_rules: vec![
                GrammarRule {
                    rule_name: "func_body_no_pass".to_string(),
                    ebnf_definition: r#"
                        func_body ::= INDENT (stmt)+ DEDENT
                        stmt ::= (expr_stmt | compound_stmt) 
                        compound_stmt ::= if_stmt | while_stmt | for_stmt | try_stmt | with_stmt | funcdef | classdef
                        # Exclude: stmt ::= pass
                    "#.to_string(),
                    enforcement: EnforcementLevel::Fatal,
                },
            ],
            forbidden_constructs: vec![
                "pass".to_string(),
                "...".to_string(), // Ellipsis
                "raise NotImplementedError()".to_string(),
            ],
        }
    }

    pub fn for_rust() -> Self {
        Self {
            language: ProgrammingLanguage::Rust,
            grammar_rules: vec![
                GrammarRule {
                    rule_name: "fn_body_no_unimplemented".to_string(),
                    ebnf_definition: r#"
                        fn_body ::= block_expr
                        block_expr ::= '{' (stmt)* (expr)? '}'
                        # Exclude: stmt ::= unimplemented!() | todo!()
                    "#.to_string(),
                    enforcement: EnforcementLevel::Fatal,
                },
            ],
            forbidden_constructs: vec![
                "unimplemented!()".to_string(),
                "todo!()".to_string(),
                "panic!(\"TODO\")".to_string(),
            ],
        }
    }
}

/// Complete sterilization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SterilizationConfig {
    pub logit_bias: LogitBias,
    pub grammar_constraint: Option<GrammarConstraint>,
    pub prompt_fencing: bool,
    pub cryptographic_delimiter: String,
    pub positive_guidance: String,
}

impl SterilizationConfig {
    pub fn default() -> Self {
        Self {
            logit_bias: LogitBias::new(),
            grammar_constraint: Some(GrammarConstraint::for_python()),
            prompt_fencing: true,
            cryptographic_delimiter: "###_STERILIZATION_PROTOCOL_v1_###".to_string(),
            positive_guidance: r#"
                If logic is complex, decompose it into helper functions.
                Do not abbreviate or omit implementation details.
                Every function must contain complete, executable logic.
                Code containing placeholders will trigger a fatal build error.
            "#.to_string(),
        }
    }

    /// Generate the sterilization prompt suffix
    pub fn generate_prompt_suffix(&self) -> String {
        format!(
            "{}\n\n{}\n\nProtocol Check: Ensure no TODOs or placeholders are present in the following output.",
            self.cryptographic_delimiter,
            self.positive_guidance
        )
    }
}

/// Tokenizer trait for converting strings to token IDs
pub trait Tokenizer {
    fn encode(&self, text: &str) -> Vec<u32>;
    fn decode(&self, token_ids: &[u32]) -> String;
}

/// Mock tokenizer implementation (would be replaced with actual tokenizer)
pub struct MockTokenizer;

impl Tokenizer for MockTokenizer {
    fn encode(&self, _text: &str) -> Vec<u32> {
        // Mock implementation - would use tiktoken or model tokenizer
        vec![]
    }

    fn decode(&self, _token_ids: &[u32]) -> String {
        String::new()
    }
}

