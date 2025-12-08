// Tier 1: Dependency-Aware Planning
// Directed Acyclic Graph (DAG) for system decomposition

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyNode {
    pub id: String,
    pub file_path: String,
    pub module_type: ModuleType,
    pub public_interface: InterfaceSpec,
    pub dependencies: Vec<String>, // IDs of dependent nodes
    pub test_plan: Option<TestPlan>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModuleType {
    Python,
    Rust,
    JavaScript,
    TypeScript,
    Config,
    Test,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceSpec {
    pub classes: Vec<ClassSignature>,
    pub functions: Vec<FunctionSignature>,
    pub constants: Vec<ConstantSignature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassSignature {
    pub name: String,
    pub methods: Vec<FunctionSignature>,
    pub docstring: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionSignature {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<String>,
    pub docstring: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub param_type: Option<String>,
    pub default: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstantSignature {
    pub name: String,
    pub value_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestPlan {
    pub unit_tests: Vec<TestCase>,
    pub integration_tests: Vec<TestCase>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub name: String,
    pub description: String,
    pub expected_behavior: String,
}

/// Dependency Graph for topological sorting and reachability analysis
#[derive(Debug, Clone)]
pub struct DependencyGraph {
    nodes: HashMap<String, DependencyNode>,
    adjacency_list: HashMap<String, Vec<String>>,
    reverse_adjacency: HashMap<String, Vec<String>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            adjacency_list: HashMap::new(),
            reverse_adjacency: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: DependencyNode) -> Result<(), String> {
        // Check for circular dependencies
        if self.would_create_cycle(&node.id, &node.dependencies) {
            return Err(format!("Adding node {} would create a circular dependency", node.id));
        }

        let deps = node.dependencies.clone();
        self.nodes.insert(node.id.clone(), node);
        
        // Build adjacency lists
        self.adjacency_list.insert(node.id.clone(), deps.clone());
        
        // Build reverse adjacency for reachability
        for dep in &deps {
            self.reverse_adjacency
                .entry(dep.clone())
                .or_insert_with(Vec::new)
                .push(node.id.clone());
        }

        Ok(())
    }

    /// Topological sort: returns nodes in dependency order
    pub fn topological_sort(&self) -> Result<Vec<String>, String> {
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        
        // Initialize in-degree for all nodes
        for node_id in self.nodes.keys() {
            in_degree.insert(node_id.clone(), 0);
        }
        
        // Calculate in-degrees
        for deps in self.adjacency_list.values() {
            for dep in deps {
                *in_degree.get_mut(dep).unwrap() += 1;
            }
        }

        // Kahn's algorithm
        let mut queue: VecDeque<String> = VecDeque::new();
        for (node_id, degree) in &in_degree {
            if *degree == 0 {
                queue.push_back(node_id.clone());
            }
        }

        let mut result = Vec::new();
        while let Some(node_id) = queue.pop_front() {
            result.push(node_id.clone());
            
            if let Some(dependents) = self.reverse_adjacency.get(&node_id) {
                for dependent in dependents {
                    let degree = in_degree.get_mut(dependent).unwrap();
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(dependent.clone());
                    }
                }
            }
        }

        // Check for cycles
        if result.len() != self.nodes.len() {
            return Err("Circular dependency detected in graph".to_string());
        }

        Ok(result)
    }

    /// Get reachable context for a node (only direct dependencies)
    pub fn get_reachable_context(&self, node_id: &str) -> Vec<InterfaceSpec> {
        let mut context = Vec::new();
        
        if let Some(node) = self.nodes.get(node_id) {
            for dep_id in &node.dependencies {
                if let Some(dep_node) = self.nodes.get(dep_id) {
                    context.push(dep_node.public_interface.clone());
                }
            }
        }
        
        context
    }

    /// Check if adding a node would create a cycle
    fn would_create_cycle(&self, new_node_id: &str, new_deps: &[String]) -> bool {
        // Check if any dependency would create a path back to new_node_id
        let mut visited = HashSet::new();
        let mut stack = Vec::new();
        
        for dep in new_deps {
            if dep == new_node_id {
                return true; // Direct self-reference
            }
            stack.push(dep.clone());
        }
        
        while let Some(current) = stack.pop() {
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current.clone());
            
            if current == new_node_id {
                return true; // Cycle detected
            }
            
            if let Some(deps) = self.adjacency_list.get(&current) {
                for dep in deps {
                    if !visited.contains(dep) {
                        stack.push(dep.clone());
                    }
                }
            }
        }
        
        false
    }

    pub fn get_node(&self, node_id: &str) -> Option<&DependencyNode> {
        self.nodes.get(node_id)
    }

    pub fn get_all_nodes(&self) -> &HashMap<String, DependencyNode> {
        &self.nodes
    }
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

