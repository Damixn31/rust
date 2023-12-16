use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: u64,
    pub description: String,
    pub priority: Priority,
    pub completed: bool,
}

impl Task {
    pub fn new(id: u64, description: &str, priority: Priority) -> Self {
        Task {
            id,
            description: description.to_string(),
            priority,
            completed: false,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }
}
