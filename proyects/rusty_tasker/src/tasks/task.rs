use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

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
    SerializationError,
    FileWriteError,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: u64,
    pub description: String,
    pub priority: Priority,
    pub creation_time: String,
    pub completed: bool,
}

impl Task {
    pub fn new(id: u64, description: &str, priority: Priority) -> Self {
        let creation_time = Self::current_time_as_string();
        Task {
            id,
            description: description.to_string(),
            priority,
            creation_time,
            completed: false,
        }
    }

    pub fn current_time_as_string() -> String {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Error al obtener la marca de tiempo actual")
            .as_secs();

        // formatear la fecha y hora
        let format_time = format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            1970 + timestamp / 31556926,      //años
            (timestamp % 31556926) / 2629743, //meses
            (timestamp % 2629743) / 86400,    //dias
            (timestamp % 86400) / 3600,       //horas
            (timestamp % 3600) / 60,          //minutos
            timestamp % 60                    //segundos
        );
        format_time
    }

    //pub fn complete(&mut self) {
    //    self.completed = true;
    //}
}
