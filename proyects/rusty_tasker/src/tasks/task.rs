use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::convert::From;
use std::fmt;

use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TaskError {
    TaskNotFound,
    InvalidPriority,
    EmptyDescription,
    SerializationError(String),
    FileWriteError(String),
    TablePrintError(String),
    InvalidIdFromat(String),
}

impl From<serde_json::Error> for TaskError {
    fn from(err: serde_json::Error) -> Self {
        TaskError::SerializationError(format!("Error al serializar la tarea: {}", err))
    }
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskError::FileWriteError(msg) => write!(f, "Error al leer el fichero: {}", msg),
            TaskError::TaskNotFound => write!(f, "La tarea no fue encontrada"),
            TaskError::EmptyDescription => write!(f, "La descripción no puede estar vacía"),
            TaskError::InvalidPriority => write!(f, "Prioridad no válida"),
            TaskError::SerializationError(err) => write!(f, "Error de serialización: {}", err),
            TaskError::TablePrintError(err) => write!(f, "Error al imprimir: {}", err),
            TaskError::InvalidIdFromat(err) => write!(f, "Error de formato: {}", err),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: u64,
    pub description: String,
    pub priority: Priority,
    pub creation_time: String,
    pub completed: bool,
    pub categories: Option<String>,
    pub tags: HashSet<String>,
}

impl Task {
    pub fn new(
        id: u64,
        description: &str,
        priority: Priority,
        categories: Option<&str>,
        tags: Option<HashSet<String>>,
    ) -> Result<Self, TaskError> {
        if description.is_empty() {
            return Err(TaskError::EmptyDescription);
        }
        let creation_time = Self::current_time_as_string();
        let new_task = Task {
            id,
            description: description.to_string(),
            priority,
            creation_time,
            completed: false,
            categories: categories.map(|c| c.to_string()),
            tags: tags.unwrap_or_default(),
        };
        Ok(new_task)
    }

    pub fn current_time_as_string() -> String {
        let local_time: DateTime<Local> = chrono::Local::now();
        let formatted = format!("{}", local_time.format("%Y-%m-%d %H:%M:%S"));
        formatted
    }
}
