// Tier 4: Compile-Fix Loop - Iterative Self-Repair

use serde::{Deserialize, Serialize};
use super::sandbox::{ValidationResult, ValidationError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflexionLoop {
    pub max_retries: u32,
    pub current_iteration: u32,
    pub repair_history: Vec<RepairContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepairContext {
    pub iteration: u32,
    pub original_code: String,
    pub validation_result: ValidationResult,
    pub error_analysis: String,
    pub repaired_code: Option<String>,
    pub success: bool,
}

impl ReflexionLoop {
    pub fn new(max_retries: u32) -> Self {
        Self {
            max_retries,
            current_iteration: 0,
            repair_history: Vec::new(),
        }
    }

    /// Execute reflexion loop: generate -> validate -> reflect -> repair
    pub fn execute<F, G>(
        &mut self,
        initial_code: String,
        validate_fn: F,
        repair_fn: G,
    ) -> Result<String, String>
    where
        F: Fn(&str) -> ValidationResult,
        G: Fn(&str, &ValidationResult) -> String,
    {
        let mut current_code = initial_code;

        loop {
            self.current_iteration += 1;

            if self.current_iteration > self.max_retries {
                return Err(format!(
                    "Max retries ({}) exceeded. Failed to repair code.",
                    self.max_retries
                ));
            }

            // Validate current code
            let validation_result = validate_fn(&current_code);

            // Create repair context
            let mut repair_context = RepairContext {
                iteration: self.current_iteration,
                original_code: current_code.clone(),
                validation_result: validation_result.clone(),
                error_analysis: self.analyze_errors(&validation_result),
                repaired_code: None,
                success: false,
            };

            // If validation passed, we're done
            if validation_result.passed {
                repair_context.success = true;
                repair_context.repaired_code = Some(current_code.clone());
                self.repair_history.push(repair_context);
                return Ok(current_code);
            }

            // Reflect on errors and generate repair
            let repaired_code = repair_fn(&current_code, &validation_result);
            repair_context.repaired_code = Some(repaired_code.clone());
            self.repair_history.push(repair_context);

            current_code = repaired_code;
        }
    }

    /// Analyze validation errors to generate actionable feedback
    fn analyze_errors(&self, validation_result: &ValidationResult) -> String {
        if validation_result.errors.is_empty() {
            return "No errors found".to_string();
        }

        let mut analysis = String::from("Validation Errors:\n");
        
        for error in &validation_result.errors {
            analysis.push_str(&format!(
                "[{}] {}: {}\n",
                match error.severity {
                    super::sandbox::ErrorSeverity::Fatal => "FATAL",
                    super::sandbox::ErrorSeverity::Error => "ERROR",
                    super::sandbox::ErrorSeverity::Warning => "WARNING",
                },
                format!("{:?}", error.error_type),
                error.message
            ));
            
            if let Some(line) = error.line {
                analysis.push_str(&format!("  Location: Line {}\n", line));
            }
        }

        analysis
    }

    /// Generate repair prompt for LLM
    pub fn generate_repair_prompt(
        &self,
        code: &str,
        validation_result: &ValidationResult,
    ) -> String {
        let error_summary = self.analyze_errors(validation_result);
        
        format!(
            r#"
###_STERILIZATION_PROTOCOL_v1_###

The following code failed the sterilization check:

```{}
{}
```

Error Details:
{}

You must fix ALL errors. Do not remove comments or TODOs - implement the missing logic.
Every function must contain complete, executable code.
Code containing placeholders will trigger a fatal build error.

Generate the complete, fixed code:
"#,
            detect_language(code),
            code,
            error_summary
        )
    }

    pub fn get_history(&self) -> &[RepairContext] {
        &self.repair_history
    }

    pub fn get_current_iteration(&self) -> u32 {
        self.current_iteration
    }
}

fn detect_language(code: &str) -> &str {
    // Simple heuristic-based language detection
    if code.contains("fn ") || code.contains("impl ") || code.contains("struct ") {
        "rust"
    } else if code.contains("def ") || code.contains("import ") || code.contains("class ") {
        "python"
    } else if code.contains("function ") || code.contains("const ") || code.contains("let ") {
        "javascript"
    } else {
        "unknown"
    }
}

