use std::fmt;

pub enum TaskError {
    TaskNotFound(u64),
    InvalidPriority(String),
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskError::TaskNotFound(id) => write!(f, "La tarea con ID {} no fue encontrada", id),
            TaskError::InvalidPriority(priority) => {
                write!(f, "Prioridad no valida: {}", priority)
            }
        }
    }
}
