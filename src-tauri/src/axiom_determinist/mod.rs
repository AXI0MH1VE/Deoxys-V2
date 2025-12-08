// AxiomDeterminist Architecture v1.0
// A Definitive Framework for Sterilized, Deterministic AI Code Generation
// Created by Alexis M. Adams

pub mod dag;
pub mod constraints;
pub mod sandbox;
pub mod reflexion;
pub mod agents;
pub mod orchestrator;

pub use dag::DependencyGraph;
pub use constraints::{LogitBias, GrammarConstraint, SterilizationConfig};
pub use sandbox::{HermeticSandbox, ValidationResult};
pub use reflexion::{ReflexionLoop, RepairContext};
pub use agents::{AgentRole, AgentState};
pub use orchestrator::Orchestrator;

/// Core sterilization policy: Zero tolerance for placeholders
pub const STERILIZATION_PROTOCOL: &str = "###_STERILIZATION_PROTOCOL_v1_###";

/// Maximum retry attempts for reflexion loop
pub const MAX_RETRIES: u32 = 10;

/// Token ban list for logit bias
pub const BANNED_TOKENS: &[&str] = &[
    "TODO", "FIXME", "XXX", "HACK",
    "todo", "fixme", "xxx", "hack",
    " TODO", " FIXME", " XXX", " HACK",
    "NotImplementedError", "NotImplemented",
    "pass", "return null", "return None",
    "omitted for brevity", "rest of code",
    "left as an exercise", "implementation omitted",
];

