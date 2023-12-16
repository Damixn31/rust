use crate::tasks::task::Task;
use serde::{Deserialize, Serialize};
use std::fs;

use super::task::Priority;

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskManager {
    pub tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, description: &str, priority: Priority) {
        let id = (self.tasks.len() + 1) as u64;
        let new_task = Task::new(id, description, priority);
        self.tasks.push(new_task);
    }

    pub fn save_tasks(&self, file_path: &str) {
        let tasks_json = serde_json::to_string(&self.tasks).expect("Error al serializar tarea");
        fs::write(file_path, tasks_json).expect("Error al escribir en el fichero");
    }

    pub fn load_tasks(file_path: &str) -> Self {
        if let Ok(tasks_json) = fs::read_to_string(file_path) {
            if let Ok(tasks) = serde_json::from_str::<Vec<Task>>(&tasks_json) {
                return TaskManager { tasks };
            } else {
                eprintln!("Error al deserializar tareas desde el fichero JSON");
            }
        }
        TaskManager::new()
    }

    pub fn delete_task(&mut self, task_id: u64) {
        self.tasks.retain(|task| task.id != task_id);
    }

    pub fn complete_task(&mut self, task_id: u64) {
        for task in &mut self.tasks {
            if task.id == task_id {
                task.completed = true;
                break;
            }
        }
    }

    pub fn uncomplete_task(&mut self, task_id: u64) {
        for task in &mut self.tasks {
            if task.id == task_id {
                task.completed = false;
                break;
            }
        }
    }

    pub fn list_tasks(&self) {
        println!("Lista de tareas:");
        for task in &self.tasks {
            let status = if task.completed {
                "Completada"
            } else {
                "Pendiente"
            };
            println!(
                "ID: {}, descripcion: {}, Estado: {}",
                task.id, task.description, status
            );
        }
        println!();
    }
}
