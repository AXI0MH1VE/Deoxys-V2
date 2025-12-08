// Orchestrator: Manages the complete AxiomDeterminist workflow

use serde::{Deserialize, Serialize};
use super::{
    dag::DependencyGraph,
    agents::*,
    reflexion::ReflexionLoop,
    sandbox::ValidationResult,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestrationResult {
    pub success: bool,
    pub generated_files: Vec<GeneratedFile>,
    pub total_iterations: u32,
    pub validation_passed: bool,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedFile {
    pub path: String,
    pub content: String,
    pub language: String,
    pub validation_passed: bool,
}

/// Main orchestrator for AxiomDeterminist workflow
pub struct Orchestrator {
    architect: ArchitectAgent,
    librarian: LibrarianAgent,
    builder: BuilderAgent,
    auditor: AuditorAgent,
    reflexion_loop: ReflexionLoop,
}

impl Orchestrator {
    pub fn new(max_retries: u32) -> Self {
        Self {
            architect: ArchitectAgent::new(),
            librarian: LibrarianAgent::new(),
            builder: BuilderAgent::new(),
            auditor: AuditorAgent::new(),
            reflexion_loop: ReflexionLoop::new(max_retries),
        }
    }

    /// Execute complete AxiomDeterminist workflow
    pub fn execute(&mut self, user_requirement: &str) -> Result<OrchestrationResult, String> {
        // Step 1: Architect generates DAG
        let mut dag = self.architect.generate_dag(user_requirement)?;
        
        // Step 2: Topological sort for execution order
        let execution_order = dag.topological_sort()?;
        
        let mut generated_files = Vec::new();
        let mut total_iterations = 0;
        let mut all_errors = Vec::new();

        // Step 3: Execute each node in dependency order
        for node_id in execution_order {
            let node = dag.get_node(&node_id)
                .ok_or_else(|| format!("Node {} not found in DAG", node_id))?;

            // Get pruned context from Librarian
            let context = self.librarian.get_pruned_context(&node_id, &dag);

            // Generate code with Builder
            let initial_code = self.builder.generate_code(node, &context)?;

            // Validate and repair with Reflexion loop
            let language = match node.module_type {
                super::dag::ModuleType::Python => "python",
                super::dag::ModuleType::Rust => "rust",
                super::dag::ModuleType::JavaScript => "javascript",
                super::dag::ModuleType::TypeScript => "typescript",
                _ => "unknown",
            };

            let final_code = match self.reflexion_loop.execute(
                initial_code,
                |code| self.auditor.validate(code, language),
                |code, validation| {
                    // Generate repair prompt and call LLM
                    self.reflexion_loop.generate_repair_prompt(code, validation)
                },
            ) {
                Ok(code) => code,
                Err(e) => {
                    all_errors.push(format!("Failed to repair {}: {}", node_id, e));
                    continue;
                }
            };

            total_iterations += self.reflexion_loop.get_current_iteration();

            // Final validation
            let final_validation = self.auditor.validate(&final_code, language);
            
            generated_files.push(GeneratedFile {
                path: node.file_path.clone(),
                content: final_code.clone(),
                language: language.to_string(),
                validation_passed: final_validation.passed,
            });

            // Index in Librarian for future context
            self.librarian.index_file(
                node.file_path.clone(),
                node.public_interface.clone(),
                node.dependencies.clone(),
            );
        }

        let validation_passed = generated_files.iter().all(|f| f.validation_passed);
        let success = validation_passed && all_errors.is_empty();

        Ok(OrchestrationResult {
            success,
            generated_files,
            total_iterations,
            validation_passed,
            errors: all_errors,
        })
    }

    /// Get status of all agents
    pub fn get_agent_statuses(&self) -> Vec<&AgentState> {
        vec![
            self.architect.get_state(),
            self.librarian.get_state(),
            self.builder.get_state(),
            self.auditor.get_state(),
        ]
    }
}

