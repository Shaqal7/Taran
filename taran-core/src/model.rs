use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Unique identifier for a Virtual User
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VirtualUserId(pub usize);

/// Iteration number for a VU
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Iteration(pub u64);

/// Result of a single request/step execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    pub step_name: String,
    pub success: bool,
    pub duration: Duration,
    pub error: Option<String>,
    pub status_code: Option<u16>,
    pub bytes_sent: u64,
    pub bytes_received: u64,
}

/// Context shared across a VU's execution
#[derive(Debug, Clone, Default)]
pub struct VirtualUserContext {
    pub id: usize,
    pub iteration: u64,
    /// Variables extracted from responses
    pub variables: std::collections::HashMap<String, String>,
}

impl VirtualUserContext {
    pub fn new(id: usize) -> Self {
        Self { id, iteration: 0, variables: std::collections::HashMap::new() }
    }

    pub fn next_iteration(&mut self) {
        self.iteration += 1;
    }

    pub fn set_variable(&mut self, key: String, value: String) {
        self.variables.insert(key, value);
    }

    pub fn get_variable(&self, key: &str) -> Option<&String> {
        self.variables.get(key)
    }
}
